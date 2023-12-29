use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

use mavio::dialects::minimal as dialect;
use mavio::io::{Read, Write};
use mavio::protocol::MavLinkVersion;
use mavio::{Frame, Receiver, Sender};

use dialect::enums::{MavAutopilot, MavModeFlag, MavState, MavType};
use dialect::Message;

/// TCP address for server and clients.
const ADDRESS: &str = ":::56001";
/// Interval between sending messages.
const SEND_INTERVAL: Duration = Duration::from_millis(500);

/// Listen to `n_iter` incoming frames and decode `HEARTBEAT` message.
fn listen<R: Read>(reader: R, whoami: String) -> mavio::errors::Result<()> {
    let mut receiver = Receiver::new(reader);

    loop {
        // Decode the entire frame
        let frame = receiver.recv()?;

        // Validate frame in the context of dialect specification (including checksum)
        if let Err(err) = frame.validate_checksum(dialect::spec()) {
            log::warn!("[{whoami}] INVALID FRAME #{}: {err:?}", frame.sequence());
            continue;
        }

        log::info!(
            "[{whoami}] FRAME #{}: mavlink_version={:?} system_id={}, component_id={}",
            frame.sequence(),
            frame.mavlink_version(),
            frame.system_id(),
            frame.component_id(),
        );

        // Decode message
        match dialect::decode(frame.payload()) {
            Ok(msg) => {
                if let Message::Heartbeat(msg) = msg {
                    log::info!("[{whoami}] HEARTBEAT #{}: {msg:?}", frame.sequence());
                } else {
                    log::info!("[{whoami}] MESSAGE #{}: {msg:?}", frame.sequence());
                    // Some other message
                }
            }
            Err(err) => {
                log::warn!("[{whoami}] DECODE ERROR #{}: {err:?}", frame.sequence());
            }
        }
    }
}

/// Sends heartbeat messages.
fn send_heartbeats<W: Write>(writer: W, whoami: String) -> mavio::errors::Result<()> {
    let mut sender = Sender::new(writer);

    // MAVLink connection settings
    let mavlink_version = MavLinkVersion::V2;
    let system_id = 15;
    let component_id = 42;
    let mut sequence: u8 = 0;

    loop {
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
            .set_sequence(sequence)
            .set_system_id(system_id)
            .set_component_id(component_id)
            .build_for(&message, mavlink_version)?;

        sender.send(&frame)?;
        log::info!("[{whoami}] FRAME #{} SENT", sequence);

        sequence = sequence.wrapping_add(1); // Increase sequence number
        thread::sleep(SEND_INTERVAL);
    }
}

/// Takes stream, sends `n` heartbeat messages, listens for incoming messages.
fn handle_stream(stream: TcpStream, whoami: String) -> mavio::errors::Result<()> {
    let reader = stream.try_clone()?;
    let recv_name = format!("{} receiver", &whoami);
    let send_name = format!("{} sender", &whoami);

    // Spawn a thread that will listen to incoming messages
    thread::spawn(move || -> mavio::errors::Result<()> { listen(reader, recv_name) });
    // Send heartbeats
    send_heartbeats(stream, send_name)
}

/// Start server and wait until it binds to address.
fn server(address: String) -> mavio::errors::Result<()> {
    // Bind to address and report (or fail)
    let listener = TcpListener::bind(address)?;

    let mut conn_n: usize = 0;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                conn_n += 1;
                log::info!("CONNECTION #{conn_n}: {}", stream.peer_addr().unwrap());
                thread::spawn(move || handle_stream(stream, format!("server #{conn_n}")));
            }
            Err(err) => {
                log::warn!("ERROR: {err:?}");
            }
        }
    }

    Ok(())
}

/// Creates a TCP server, listens to incoming messages, sends heartbeat messages to each client.
///
/// Use `tcp_client` from library examples to connect to this server.
fn main() {
    // Setup logger
    env_logger::builder()
        .filter_level(log::LevelFilter::Info) // Suppress everything below `info` for third-party modules.
        .filter_module(env!("CARGO_PKG_NAME"), log::LevelFilter::Trace) // Allow everything from current package
        .init();

    server(ADDRESS.to_string()).unwrap();
}
