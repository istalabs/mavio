use mavio::protocol::V2;
use mavio::Frame;
use mavspec::rust::derive::Message;

use dialect::enums::{MavFrame, MavType};
use mavio_examples_custom::dialects::common as dialect;

const FIVE: usize = 5;

#[derive(Clone, Debug, Message)]
#[crc_extra(42)]
#[message_id(255)]
struct CustomMessage {
    field_u8: u8,
    array_field_u8: [u8; 10],

    #[base_type(u8)] // Type of field
    mav_frame: MavFrame,

    #[repr_type(u8)] // Base type of enum
    #[base_type(u16)] // Base type of field (can be larger than enum)
    mav_type: [MavType; FIVE],
}

fn play_with_custom_message() {
    // Create message
    let message = {
        let mut message = CustomMessage {
            mav_frame: MavFrame::BodyFrd,
            ..Default::default()
        };

        message.array_field_u8[5] = 7;
        message.mav_type[3] = MavType::Camera;

        message
    };
    log::info!("Message: {message:#?}");

    // Create frame
    let frame = Frame::builder()
        .sequence(0)
        .system_id(10)
        .component_id(10)
        .version(V2)
        .message(&message)
        .unwrap()
        .build();
    log::info!("Frame: {frame:#?}");
}

fn main() {
    // Setup logger
    env_logger::builder()
        .filter_level(log::LevelFilter::Info) // Suppress everything below `info` for third-party modules.
        .filter_module(env!("CARGO_PKG_NAME"), log::LevelFilter::Trace) // Allow everything from current package
        .init();

    play_with_custom_message()
}

#[cfg(test)]
#[test]
fn test_filtered_common_dialect() {
    play_with_custom_message()
}
