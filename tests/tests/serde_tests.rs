#[cfg(feature = "serde")]
mod serde_tests {
    #[test]
    #[cfg(feature = "unstable")]
    fn test_basic_errors_serialization() {
        use mavio::error::*;

        let err = IoError::from(IoErrorKind::UnexpectedEof);
        assert_eq!(
            serde_json::to_string(&err).unwrap(),
            "{\"kind\":\"UnexpectedEof\"}"
        );
    }

    #[test]
    #[cfg(feature = "unstable")]
    #[cfg(feature = "embedded-io")]
    #[cfg(not(feature = "std"))]
    fn test_embedded_io_errors_serialization() {
        use mavio::error::*;

        let err = IoError::from(embedded_io::ErrorKind::NotFound);
        assert_eq!(
            serde_json::to_string(&err).unwrap(),
            "{\"kind\":{\"Embedded\":null}}"
        );
    }

    #[test]
    #[cfg(feature = "unstable")]
    #[cfg(feature = "embedded-io")]
    #[cfg(feature = "std")]
    fn test_embedded_io_errors_serialization_std() {
        use mavio::error::*;

        let err = IoError::from(embedded_io::ErrorKind::NotFound);
        assert_eq!(
            serde_json::to_string(&err).unwrap(),
            "{\"kind\":{\"Embedded\":null}}"
        );
    }

    #[test]
    #[cfg(feature = "unstable")]
    #[cfg(feature = "std")]
    fn test_std_io_errors_serialization() {
        use mavio::error::*;

        let err = IoError::from(std::io::Error::new(std::io::ErrorKind::NotFound, "foo"));
        assert_eq!(
            serde_json::to_string(&err).unwrap(),
            "{\"kind\":{\"Std\":[\"NotFound\"]},\"error\":\"foo\"}"
        );
    }
}
