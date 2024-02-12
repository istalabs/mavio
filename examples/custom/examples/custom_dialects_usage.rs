use mavio::protocol::V2;
use mavio::utils::SliceWriter;
use mavio::{Frame, Sender};

// Import a subset of MAVLink `common` dialect
use mavio_examples_custom::dialects::common as dialect;
// We have a `COMMAND_INT` message since we've requested commands.
use dialect::messages::CommandInt;
// `MAV_CMD` has been retained as it contains commands.
use dialect::enums::MavCmd;
// We also have the following enums since the commands we've requested require them.
use dialect::enums::MavFrame;

fn write_to_buffer() {
    // Create a `MAV_CMD_DO_SET_ROI_LOCATION` command.
    let message = CommandInt {
        target_system: 10,
        target_component: 20,
        frame: MavFrame::Global,
        command: MavCmd::DoSetRoiLocation,
        current: 0,
        autocontinue: 0,
        param1: 1.0,
        param2: 0.0,
        param3: 0.0,
        param4: 0.0,
        x: 1000,
        y: 2000,
        z: 100.0,
    };
    log::info!("Message: {message:#?}");

    // Construct a MAVLink 2 frame
    let frame = Frame::builder()
        .sequence(0)
        .system_id(10)
        .component_id(10)
        .version(V2)
        .message(&message)
        .unwrap()
        .versionless();
    log::info!("Frame: {message:#?}");

    // Write MAVLink frame to a buffer
    let mut buf = [0u8; 44];
    let mut sender = Sender::versionless(SliceWriter::new(&mut buf));
    sender.send(&frame).unwrap();
    log::info!("Buffer: {buf:?}");
}

fn main() {
    // Setup logger
    env_logger::builder()
        .filter_level(log::LevelFilter::Info) // Suppress everything below `info` for third-party modules.
        .filter_module(env!("CARGO_PKG_NAME"), log::LevelFilter::Trace) // Allow everything from current package
        .init();

    write_to_buffer();
}

#[cfg(test)]
#[test]
fn test_filtered_common_dialect() {
    write_to_buffer()
}
