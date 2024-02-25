use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use mavio::dialects::minimal as dialect;
use mavio::io::{Read, Write};
use mavio::protocol::{Dialect, V2};
use mavio::{Frame, Receiver, Sender};

use dialect::enums::{MavAutopilot, MavModeFlag, MavState, MavType};
use dialect::Minimal;

/// TCP address for server and clients.
const ADDRESS: &str = ":::56001";
/// Interval between sending messages. Increase for slo-mo.
const SEND_INTERVAL: Duration = Duration::from_millis(50);
/// Number of messages sent before stopping.
const N_ITER: usize = 10;
/// Number of clients.
const N_CLIENTS: usize = 5;

/// Listen to `n_iter` incoming frames and decode `HEARTBEAT` message.
fn listen<R: Read>(reader: R, whoami: String, n_iter: usize) -> mavio::errors::Result<()> {
    let mut receiver = Receiver::versionless(reader);

    for _ in 0..n_iter {
        // Decode the entire frame
        let frame = receiver.recv()?;

        // Validate frame in the context of dialect specification (including checksum)
        if let Err(err) = frame.validate_checksum::<Minimal>() {
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
                if let Minimal::Heartbeat(msg) = msg {
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
fn send_heartbeats<W: Write>(
    writer: W,
    whoami: String,
    n_iter: usize,
) -> mavio::errors::Result<()> {
    // Use a versionless sender that accepts both `MAVLink 1` and `MAVLink 2` frames.
    let mut sender = Sender::versionless(writer);

    // MAVLink connection settings
    let mavlink_version = V2;
    let system_id = 15;
    let component_id = 42;
    let mut sequence: u8 = 0;

    for _ in 0..n_iter {
        // Define message
        let message = dialect::messages::Heartbeat {
            type_: MavType::FixedWing,
            autopilot: MavAutopilot::Generic,
            base_mode: MavModeFlag::TEST_ENABLED & MavModeFlag::CUSTOM_MODE_ENABLED,
            custom_mode: 0,
            system_status: MavState::Active,
            mavlink_version: Minimal::version().unwrap_or(0),
        };

        // Build frame from message
        let frame = Frame::builder()
            .sequence(sequence)
            .system_id(system_id)
            .component_id(component_id)
            .version(mavlink_version)
            .message(&message)?
            .build()
            .versionless();

        if let Err(err) = sender.send(&frame) {
            log::warn!("[{whoami}] SEND ERROR #{}: {err:?}", frame.sequence());
            continue;
        }

        log::info!("[{whoami}] FRAME #{} SENT", sequence);

        sequence = sequence.wrapping_add(1); // Increase sequence number
        thread::sleep(SEND_INTERVAL);
    }

    Ok(())
}

/// Connect to server, listen to incoming messages, send `n_iter` heartbeats.
fn client(address: String, whoami: String, n_iter: usize) -> mavio::errors::Result<String> {
    let client = TcpStream::connect(address)?;
    handle_stream(client, whoami.clone(), n_iter)?;

    Ok(whoami)
}

/// Takes stream, sends `n` heartbeat messages, listens for incoming messages.
fn handle_stream(stream: TcpStream, whoami: String, n_iter: usize) -> mavio::errors::Result<()> {
    let reader = stream.try_clone()?;
    let recv_name = format!("{} receiver", &whoami);
    let send_name = format!("{} sender", &whoami);

    // Spawn a thread that will listen to incoming messages
    thread::spawn(move || -> mavio::errors::Result<()> { listen(reader, recv_name, n_iter) });
    // Send heartbeats
    send_heartbeats(stream, send_name, n_iter)
}

/// Starts server, reports via [`mpsc`] once bound to address, listens to incoming connections,
/// sends `n_iter` heartbeats to each.   
fn server(address: String, tx: mpsc::Sender<mavio::errors::Result<()>>, n_iter: usize) {
    // Bind to address and report (or fail)
    let listener = match TcpListener::bind(address) {
        Ok(listener) => listener,
        Err(err) => {
            tx.send(Err(err.into())).unwrap();
            return;
        }
    };
    tx.send(Ok(())).unwrap();

    let mut conn_n: usize = 0;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                conn_n += 1;
                log::info!("CONNECTION #{conn_n}: {}", stream.peer_addr().unwrap());
                thread::spawn(move || handle_stream(stream, format!("server #{conn_n}"), n_iter));
            }
            Err(err) => {
                log::warn!("ERROR: {err:?}");
            }
        }
    }
}

/// Start server and wait until it binds to address.
fn start_server(address: String, n_iter: usize) -> mavio::errors::Result<()> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || server(address, tx, n_iter));
    rx.recv().unwrap()
}

/// Spawns `n_client` that will connect to server and send `n_iter` heartbeats.
fn spawn_clients(address: String, n_clients: usize, n_iter: usize) {
    // Spawn clients
    let (tx, rx) = mpsc::channel();
    for i in 0..n_clients {
        let channel = tx.clone();
        let address = address.clone();
        thread::spawn(move || {
            channel
                .send(client(address, format!("client #{i}"), n_iter))
                .unwrap();
        });
    }
    // Await clients to complete their jobs, then exit
    for _ in 0..n_clients {
        match rx.recv().unwrap() {
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
fn main() {
    // Setup logger
    env_logger::builder()
        .filter_level(log::LevelFilter::Info) // Suppress everything below `info` for third-party modules.
        .filter_module(env!("CARGO_PKG_NAME"), log::LevelFilter::Trace) // Allow everything from current package
        .init();

    start_server(ADDRESS.to_string(), N_ITER).unwrap();
    spawn_clients(ADDRESS.to_string(), N_CLIENTS, N_ITER);
}

#[cfg(test)]
#[test]
fn tcp_ping_pong() {
    let port = portpicker::pick_unused_port().unwrap();

    start_server(format!(":::{port}"), 5).unwrap();
    spawn_clients(format!(":::{port}"), 2, 5);
}
