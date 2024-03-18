use crate::protocol::{Checksum, CrcExtra, Header, Signature};

use crate::prelude::*;

/// <sup>`⚠`</sup>
/// Implementors of this trait can update frame data in-place.
///
/// This trait contains blanket implementations for utility methods, that rely on the
/// [`UpdateFrameUnsafe`]. The latter has to be implemented in order to update frames.
///
/// # Examples
///
/// Create a frame updater, that inverts bits of a payload.
///
/// ```rust
/// use mavio::prelude::*;
/// use mavio::protocol::{Checksum, Header, Signature, UpdateFrame, UpdateFrameUnsafe};
///
/// struct Flipper;
///
/// impl<V: MaybeVersioned> UpdateFrameUnsafe<V> for Flipper {
///     unsafe fn update_unsafe(
///         &mut self,
///         header: Header<V>,
///         payload: &mut [u8],
///         checksum: &mut Checksum,
///         signature: &mut Option<Signature>
///     ) {
///         for i in 0..payload.len() {
///             payload[i] = payload[i] ^ 0xff;
///         }
///     }
/// }
/// impl<V: MaybeVersioned> UpdateFrame<V> for Flipper {}
///
/// let mut frame = Frame::builder()
///     .version(V2)
///     /* frame settings */
/// #    .sequence(0)
/// #    .system_id(0)
/// #    .component_id(0)
/// #    .message_id(0)
///     .payload(&[0, 1, 255])
/// #    .crc_extra(0)
///     .build();
///
/// let mut flipper = Flipper;
/// flipper.update(&mut frame, 42);
///
/// assert_eq!(frame.payload().bytes(), &[255, 254, 0]);
/// ```
pub trait UpdateFrame<V: MaybeVersioned>: UpdateFrameUnsafe<V> {
    /// <sup>`⚠`</sup>
    /// Updates a frame.
    ///
    /// **⚠** This method relies on the access to internal frame state and can't be reimplemented!
    ///
    /// The `crc_extra` parameter will be used to calculate a correct checksum.
    ///
    /// If frame updater adds a signature, then `MAVLink 2` frame will be considered signed (and
    /// vice versa). For `MAVLink 1` frames adding signature won't have any effect on a frame.
    ///
    /// **⚠** If the underlying [`UpdateFrameUnsafe::update_unsafe`] does not re-calculate
    /// [`Frame::signature`], then it may be potentially corrupted. Make sure, that you know, how to
    /// sign the updated frame afterward.
    fn update(&mut self, frame: &mut Frame<V>, crc_extra: CrcExtra) {
        unsafe {
            self.update_unchecked(frame);
        }

        frame.checksum = frame.calculate_crc(crc_extra);
    }

    /// <sup>`⚠`</sup>
    /// Updates frame without changing [`Frame::checksum`] to the correct value.
    ///
    /// **⚠** This method relies on the access to internal frame state and can't be reimplemented!
    ///
    /// If frame updater adds a signature, then `MAVLink 2` frame will be considered signed (and
    /// vice versa). For `MAVLink 1` frames adding signature won't have any effect on a frame.
    ///
    /// **⚠** The implementor of the trait has a full responsibility for providing a correct
    /// checksum and signature. Always use [UpdateFrame::update], if you know `CRC_EXTRA` to
    /// calculate a correct checksum.
    unsafe fn update_unchecked(&mut self, frame: &mut Frame<V>) {
        self.update_unsafe(
            frame.header.clone(),
            frame.payload.bytes_mut(),
            &mut frame.checksum,
            &mut frame.signature,
        );

        match frame.version() {
            MavLinkVersion::V1 => frame.signature = None,
            MavLinkVersion::V2 => {
                frame.header.set_is_signed(frame.signature.is_some());
            }
        }
    }
}

/// <sup>`⚠`</sup>
/// This trait should be implemented in order to use [`UpdateFrame`] trait and update frames
/// in-place.
///
/// **⚠** This trait contains unsafe and dangerous methods, that may corrupt frames and lead to
/// undefined behavior. In general, you would never need such low-level access to frame internals.
/// However, in some scenarios this may be the only way to provide a desired functionality. For
/// example, if you want to encrypt frames keeping them compatible with existing MAVLink
/// network infrastructure.
pub trait UpdateFrameUnsafe<V: MaybeVersioned> {
    /// <sup>`⚠`</sup>
    /// Updates frame payload bytes in-place, checksum, and signature.
    ///
    /// **⚠** The method is considered unsafe, as it may lead to data corruption and undefined
    /// behavior. You should almost never use it directly. Instead, use [`UpdateFrame`] trait, that
    /// contains blanket implementations for most of the cases, when you may expect sane results.
    ///
    /// If you just want to update an existing frame with data from a MAVLink message, then it is
    /// always safer to use [`Frame::try_update_from`].
    unsafe fn update_unsafe(
        &mut self,
        header: Header<V>,
        payload: &mut [u8],
        checksum: &mut Checksum,
        signature: &mut Option<Signature>,
    );
}
