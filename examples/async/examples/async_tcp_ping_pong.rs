use std::time::Duration;

use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio::sync::oneshot;

use mavio::dialects::minimal as dialect;
use mavio::protocol::V2;
use mavio::{AsyncReceiver, AsyncSender, Frame};

use dialect::enums::{MavAutopilot, MavModeFlag, MavState, MavType};
use dialect::Message;

/// TCP address for server and clients.
const ADDRESS: &str = ":::56001";
/// Interval between sending messages. Increase for slo-mo.
const SEND_INTERVAL: Duration = Duration::from_millis(50);
/// Number of messages sent before stopping.
const N_ITER: usize = 10;
/// Number of clients.
const N_CLIENTS: usize = 5;

/// Listen to `n_iter` incoming frames and decode `HEARTBEAT` message.
async fn listen<R: AsyncRead + Unpin>(
    reader: R,
    whoami: String,
    n_iter: usize,
) -> mavio::errors::Result<()> {
    // Use a versioned `AsyncReceiver` that will accept only messages of a specified MAVLink version
    let mut receiver = AsyncReceiver::versioned(reader, V2);

    for _ in 0..n_iter {
        // Decode the entire frame
        let frame = receiver.recv_frame().await?;

        // Validate frame in the context of dialect specification (including checksum)
        if let Err(err) = frame.validate_checksum(dialect::spec()) {
            log::warn!("[{whoami}] INVALID FRAME #{}: {err:?}", frame.sequence());
            continue;
        }

        log::info!(
            "[{whoami}] FRAME #{}: mavlink_version={:?} system_id={}, component_id={}",
            frame.sequence(),
            frame.version(),
            frame.system_id(),
            frame.component_id(),
        );

        // Decode message
        match frame.decode() {
            Ok(msg) => {
                if let Message::Heartbeat(msg) = msg {
                    log::info!("[{whoami}] HEARTBEAT #{}: {msg:?}", frame.sequence());
                } else {
                    // Some other message
                    log::info!("[{whoami}] MESSAGE #{}: {msg:?}", frame.sequence());
                }
            }
            Err(err) => {
                log::warn!("[{whoami}] DECODE ERROR #{}: {err:?}", frame.sequence());
            }
        }
    }

    Ok(())
}

/// Send `n_iter` heartbeat messages, then stops.
async fn send_heartbeats<W: AsyncWrite + Unpin>(
    writer: W,
    whoami: String,
    n_iter: usize,
) -> mavio::errors::Result<()> {
    // MAVLink connection settings
    let mavlink_version = V2;
    let system_id = 15;
    let component_id = 42;
    let mut sequence: u8 = 0;

    // Use a versioned `AsyncSender` that will accept only messages of a specified MAVLink version
    let mut sender = AsyncSender::versioned(writer, V2);

    for _ in 0..n_iter {
        // Define message
        let message = dialect::messages::Heartbeat {
            type_: MavType::FixedWing,
            autopilot: MavAutopilot::Generic,
            base_mode: MavModeFlag::TEST_ENABLED & MavModeFlag::CUSTOM_MODE_ENABLED,
            custom_mode: 0,
            system_status: MavState::Active,
            mavlink_version: dialect::spec().version().unwrap_or(0),
        };

        // Build frame from message
        let frame = Frame::builder()
            .sequence(sequence)
            .system_id(system_id)
            .component_id(component_id)
            .version(mavlink_version)
            .message(&message)?
            .build();

        if let Err(err) = sender.send_frame(&frame).await {
            log::warn!("[{whoami}] SEND ERROR #{}: {err:?}", frame.sequence());
            continue;
        }

        log::info!("[{whoami}] FRAME #{} SENT", sequence);

        sequence = sequence.wrapping_add(1); // Increase sequence number
        tokio::time::sleep(SEND_INTERVAL).await;
    }

    Ok(())
}

/// Connect to server, listen to incoming messages, send `n_iter` heartbeats.
async fn client(address: String, whoami: String, n_iter: usize) -> mavio::errors::Result<String> {
    let client = TcpStream::connect(address).await?;
    handle_stream(client, whoami.clone(), n_iter).await?;

    Ok(whoami)
}

/// Takes stream, sends `n` heartbeat messages, listens for incoming messages.
async fn handle_stream(
    stream: TcpStream,
    whoami: String,
    n_iter: usize,
) -> mavio::errors::Result<()> {
    // Tokio provides `into_split` instead of `try_clone` in `std::net` counterpart.
    let (reader, writer) = stream.into_split();

    let recv_name = format!("{} receiver", &whoami);
    let send_name = format!("{} sender", &whoami);

    // Spawn a thread that will listen to incoming messages
    tokio::spawn(async move { listen(reader, recv_name, n_iter).await });
    // Send heartbeats
    send_heartbeats(writer, send_name, n_iter).await
}

/// Starts server, reports via [`mpsc`] once bound to address, listens to incoming connections,
/// sends `n_iter` heartbeats to each.   
async fn server(address: String, tx: oneshot::Sender<mavio::errors::Result<()>>, n_iter: usize) {
    // Bind to address and report (or fail)
    let listener = match TcpListener::bind(address).await {
        Ok(listener) => listener,
        Err(err) => {
            tx.send(Err(err.into())).unwrap();
            return;
        }
    };
    tx.send(Ok(())).unwrap();

    let mut conn_n: usize = 0;

    loop {
        match listener.accept().await {
            Ok((stream, addr)) => {
                conn_n += 1;
                log::info!("CONNECTION #{conn_n}: {}", addr);
                tokio::spawn(async move {
                    handle_stream(stream, format!("server #{conn_n}"), n_iter).await
                });
            }
            Err(err) => {
                log::warn!("ERROR: {err:?}");
            }
        }
    }
}

/// Start server and wait until it binds to address.
async fn start_server(address: String, n_iter: usize) -> mavio::errors::Result<()> {
    let (tx, rx) = oneshot::channel();
    tokio::spawn(async move { server(address, tx, n_iter).await });
    rx.await.unwrap()
}

/// Spawns `n_client` that will connect to server and send `n_iter` heartbeats.
async fn spawn_clients(address: String, n_clients: usize, n_iter: usize) {
    // Spawn clients
    let (tx, mut rx) = mpsc::channel(N_CLIENTS / 2);
    for i in 0..n_clients {
        let channel = tx.clone();
        let address = address.clone();
        tokio::spawn(async move {
            channel
                .send(client(address, format!("client #{i}"), n_iter).await)
                .await
                .unwrap();
        });
    }
    // Await clients to complete their jobs, then exit
    for _ in 0..n_clients {
        match rx.recv().await.unwrap() {
            Ok(whoami) => {
                log::info!("FINISHED: {whoami}.");
            }
            Err(err) => {
                log::error!("Client ERROR: {err:?}");
            }
        }
    }
}

/// Creates a TCP server and spawns [`N_CLIENTS`] clients. Each sends and receives [`N_ITER`] of heartbeat messages.
#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Setup logger
    env_logger::builder()
        .filter_level(log::LevelFilter::Info) // Suppress everything below `info` for third-party modules.
        .filter_module(env!("CARGO_PKG_NAME"), log::LevelFilter::Trace) // Allow everything from current package
        .init();

    start_server(ADDRESS.to_string(), N_ITER).await.unwrap();
    spawn_clients(ADDRESS.to_string(), N_CLIENTS, N_ITER).await;
}

#[cfg(test)]
#[tokio::test]
async fn async_tcp_ping_pong() {
    let port = portpicker::pick_unused_port().unwrap();

    start_server(format!(":::{port}"), 5).await.unwrap();
    spawn_clients(format!(":::{port}"), 2, 5).await;
}
