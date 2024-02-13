#[cfg(feature = "minimal")]
mod needs_dialect {
    use dialect::messages::Heartbeat;
    use mavio::dialects::minimal as dialect;
    use mavio::dialects::minimal::enums::{MavAutopilot, MavModeFlag, MavState, MavType};
    use mavio::protocol::{Dialectless, Versioned, V1, V2};
    use mavio::utils::{SliceReader, SliceWriter};
    use mavio::{Frame, Receiver, Sender};

    fn default_heartbeat_message() -> Heartbeat {
        Heartbeat {
            type_: MavType::FixedWing,
            autopilot: MavAutopilot::Generic,
            base_mode: MavModeFlag::TEST_ENABLED & MavModeFlag::CUSTOM_MODE_ENABLED,
            custom_mode: 0,
            system_status: MavState::Active,
            mavlink_version: dialect::spec().version().unwrap_or(0),
        }
    }

    fn default_heartbeat_frame<V: Versioned>(version: V) -> Frame<V, Dialectless> {
        Frame::builder()
            .version(version)
            .sequence(0)
            .system_id(1)
            .component_id(42)
            .message(&default_heartbeat_message())
            .unwrap()
            .build()
    }

    fn assert_default_frame<V: Versioned>(frame: Frame<V, Dialectless>) {
        assert_eq!(frame.sequence(), 0);
        assert_eq!(frame.system_id(), 1);
        assert_eq!(frame.component_id(), 42);
        assert!(V::matches(frame.version()));

        if let dialect::Message::Heartbeat(message) = frame.decode().unwrap() {
            let expected_message = default_heartbeat_message();
            assert_eq!(message.mavlink_version, expected_message.mavlink_version);
            assert_eq!(message.autopilot as u8, expected_message.autopilot as u8);
            assert_eq!(message.base_mode.bits(), expected_message.base_mode.bits());
            assert_eq!(message.custom_mode, expected_message.custom_mode);
            assert_eq!(
                message.system_status as u8,
                expected_message.system_status as u8
            );
            assert_eq!(message.mavlink_version, expected_message.mavlink_version);
        } else {
            panic!("Incorrect message: {frame:?}");
        }
    }

    fn v1_v2_frames_buffer() -> ([u8; 255], usize) {
        let frame_v1 = default_heartbeat_frame(V1);
        let frame_v2 = default_heartbeat_frame(V2);

        let mut buffer = [0u8; 255];

        let mut sender_v1 = Sender::new(SliceWriter::new(buffer.as_mut_slice()));
        let frame_v2_offset = sender_v1.send_frame(&frame_v1).unwrap();

        let mut sender_v2 = Sender::new(SliceWriter::new(
            &mut buffer.as_mut_slice()[frame_v2_offset..],
        ));
        sender_v2.send_frame(&frame_v2).unwrap();

        (buffer, frame_v2_offset)
    }

    #[test]
    fn test_write_versionless_read_versionless() {
        let mut buffer = [0u8; 255];

        let mut sender = Sender::versionless(SliceWriter::new(buffer.as_mut_slice()));

        sender
            .send_frame(&default_heartbeat_frame(V1).versionless())
            .unwrap();
        sender
            .send_frame(&default_heartbeat_frame(V2).versionless())
            .unwrap();

        let mut receiver = Receiver::versionless(SliceReader::new(buffer.as_slice()));

        let frame_v1 = receiver.recv_frame().unwrap().try_versioned(V1).unwrap();
        let frame_v2 = receiver.recv_frame().unwrap().try_versioned(V2).unwrap();

        assert_default_frame(frame_v1);
        assert_default_frame(frame_v2);
    }

    #[test]
    fn test_write_versioned_read_versionless() {
        let (buffer, _) = v1_v2_frames_buffer();

        let mut receiver = Receiver::versionless(SliceReader::new(buffer.as_slice()));

        let frame_v1 = receiver.recv_frame().unwrap().try_versioned(V1).unwrap();
        let frame_v2 = receiver.recv_frame().unwrap().try_versioned(V2).unwrap();

        assert_default_frame(frame_v1);
        assert_default_frame(frame_v2);
    }

    #[test]
    fn test_write_versioned_read_versioned() {
        let (buffer, frame_v2_offset) = v1_v2_frames_buffer();

        let mut receiver_v1 = Receiver::new::<V1>(SliceReader::new(buffer.as_slice()));
        let frame = receiver_v1.recv_frame().unwrap();
        assert_default_frame(frame);
        assert!(receiver_v1.recv_frame().is_err());

        let mut receiver_v2 =
            Receiver::new::<V2>(SliceReader::new(&buffer.as_slice()[frame_v2_offset..]));
        let frame = receiver_v2.recv_frame().unwrap();
        assert_default_frame(frame);
        assert!(receiver_v2.recv_frame().is_err());

        let mut receiver_v2_first_frame =
            Receiver::new::<V2>(SliceReader::new(&buffer.as_slice()[0..frame_v2_offset]));
        assert!(receiver_v2_first_frame.recv_frame().is_err());
    }
}
