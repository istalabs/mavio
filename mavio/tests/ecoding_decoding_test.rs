#[cfg(feature = "dlct-minimal")]
#[cfg(feature = "extras")]
mod needs_dialect {
    use dialect::messages::Heartbeat;
    use mavio::dialects::minimal as dialect;
    use mavio::prelude::*;

    #[test]
    fn test_direct_frame_decoding() {
        let frame = Frame::builder()
            .version(V2)
            .system_id(1)
            .component_id(0)
            .sequence(0)
            .message(&Heartbeat::default())
            .unwrap()
            .build();

        match frame.message_id() {
            Heartbeat::ID => {
                Heartbeat::try_from(frame.payload()).unwrap();
            }
            _ => unreachable!(),
        };
    }
}
