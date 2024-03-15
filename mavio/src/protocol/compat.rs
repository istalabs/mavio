use crate::error::IncompatFlagsError;
use crate::protocol::{CompatFlags, IncompatFlags};

use crate::prelude::*;

/// Manages frame compatibility.
///
/// Defines MAVLink compatibility and incompatibility flags and how to process messages with or
/// without these flags.
///
/// # Usage
///
/// ```rust
/// use mavio::protocol::{CompatFlags, CompatProcessor, IncompatFlags, CompatStrategy};
/// use mavio::prelude::*;
///
/// let processor = CompatProcessor::builder()
///     // Set compatibility flags
///     .compat_flags(CompatFlags::BIT_1 | CompatFlags::BIT_2)
///     // Set incompatibility flags
///     .incompat_flags(IncompatFlags::BIT_4 | IncompatFlags::BIT_5)
///     // Set these flags for all outgoing messages
///     .outgoing(CompatStrategy::Enforce)
///     // Reject messages which do not comply with these flags
///     .incoming(CompatStrategy::Reject)
///     // Ignore message signing incompatibility flag
///     .ignore_signature(true)
///     // Build the manager
///     .build();
///
/// let mut frame = Frame::builder()
///     .version(V2)
///     /* frame settings */
/// #    .sequence(0)
/// #    .system_id(0)
/// #    .component_id(0)
/// #    .message_id(0)
/// #    .payload(&[0;0])
/// #    .crc_extra(0)
///     .build();
///
/// processor.process_outgoing(&mut frame).unwrap();
///
/// assert!(frame.incompat_flags().contains(IncompatFlags::BIT_4 | IncompatFlags::BIT_5));
/// ```
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CompatProcessor {
    incompat_flags: Option<IncompatFlags>,
    compat_flags: Option<CompatFlags>,
    incoming: CompatStrategy,
    outgoing: CompatStrategy,
    ignore_signature: bool,
}

/// Defines, how to process compatibility and incompatibility flags.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CompatStrategy {
    /// Reject messages, that are not compatible with the incompatibility flags, keep compatibility
    /// flags as they are.
    Reject,
    /// Reject messages, that are not compatible with the incompatibility flags, set compatibility
    /// flags to the defined values.
    RejectSet,
    /// Set both incompatibility and compatibility flags to the defined values.
    Enforce,
    /// Set both compatibility flags to the defined values, proxy compatibility flags.
    EnforceProxy,
    /// Proxy flags as they are.
    Proxy,
}

/// A trait for entities, that can be converted to [`CompatProcessor`].
///
/// This trait is currently implemented for [`CompatProcessor`] and [`CompatProcessorBuilder`].
pub trait IntoCompatProcessor {
    /// Converts entity into a [`CompatProcessor`].
    fn into_compat_processor(self) -> CompatProcessor;
}

/// Builder for [`CompatProcessor`].
#[derive(Clone, Debug)]
pub struct CompatProcessorBuilder {
    inner: CompatProcessor,
}

impl CompatProcessor {
    /// Creates a builder populated with default values.
    #[inline(always)]
    pub fn builder() -> CompatProcessorBuilder {
        CompatProcessorBuilder::default()
    }

    /// Converts [`CompatProcessor`] into a [`CompatProcessorBuilder`].
    #[inline(always)]
    pub fn update(self) -> CompatProcessorBuilder {
        CompatProcessorBuilder { inner: self }
    }

    /// Incompatibility flags.
    ///
    /// Default value is [`None`].
    #[inline(always)]
    pub fn incompat_flags(&self) -> Option<IncompatFlags> {
        self.incompat_flags
    }

    /// Compatibility flags.
    ///
    /// Default value is [`None`].
    #[inline(always)]
    pub fn compat_flags(&self) -> Option<CompatFlags> {
        self.compat_flags
    }

    /// Incoming strategy.
    ///
    /// Default value is [`CompatStrategy::Reject`].
    #[inline(always)]
    pub fn incoming(&self) -> CompatStrategy {
        self.incoming
    }

    /// Outgoing strategy.
    ///
    /// Default value is [`CompatStrategy::Enforce`].
    #[inline(always)]
    pub fn outgoing(&self) -> CompatStrategy {
        self.outgoing
    }

    /// Whether [`IncompatFlags::MAVLINK_IFLAG_SIGNED`] should be ignored.
    ///
    /// Default value is `true`.
    #[inline(always)]
    pub fn ignore_signature(&self) -> bool {
        self.ignore_signature
    }

    /// Takes incoming frame and processes it according to a [`Self::incoming`] strategy.
    #[inline(always)]
    pub fn process_incoming<V: MaybeVersioned>(
        &self,
        frame: &mut Frame<V>,
    ) -> core::result::Result<(), IncompatFlagsError> {
        self.process_for_strategy(frame, self.incoming)
    }

    /// Takes outgoing frame and processes it according to a [`Self::outgoing`] strategy.
    #[inline(always)]
    pub fn process_outgoing<V: MaybeVersioned>(
        &self,
        frame: &mut Frame<V>,
    ) -> core::result::Result<(), IncompatFlagsError> {
        self.process_for_strategy(frame, self.outgoing)
    }

    /// Processes a [`Frame`] given the provided strategy.
    pub fn process_for_strategy<V: MaybeVersioned>(
        &self,
        frame: &mut Frame<V>,
        strategy: CompatStrategy,
    ) -> core::result::Result<(), IncompatFlagsError> {
        if frame.matches_version(V2) {
            if let Some(compat_flags) = self.compat_flags {
                match strategy {
                    CompatStrategy::Enforce | CompatStrategy::RejectSet => {
                        frame.header.compat_flags = compat_flags;
                    }
                    _ => {}
                }
            }

            if let Some(mut incompat_flags) = self.incompat_flags {
                if self.ignore_signature {
                    incompat_flags.set(IncompatFlags::MAVLINK_IFLAG_SIGNED, frame.is_signed());
                }

                match strategy {
                    CompatStrategy::Reject | CompatStrategy::RejectSet => {
                        if incompat_flags != frame.header.incompat_flags {
                            return Err(IncompatFlagsError {
                                expected: incompat_flags,
                                actual: frame.header.incompat_flags,
                            });
                        }
                    }
                    CompatStrategy::Enforce | CompatStrategy::EnforceProxy => {
                        frame.header.incompat_flags = incompat_flags;
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }
}

impl IntoCompatProcessor for CompatProcessor {
    /// Passes [`CompatProcessor`] without change.
    fn into_compat_processor(self) -> CompatProcessor {
        self
    }
}

impl CompatProcessorBuilder {
    /// Creates a new [`CompatProcessorBuilder`] with default settings.
    pub fn new() -> CompatProcessorBuilder {
        CompatProcessorBuilder {
            inner: CompatProcessor {
                incompat_flags: None,
                compat_flags: None,
                incoming: CompatStrategy::Reject,
                outgoing: CompatStrategy::Enforce,
                ignore_signature: true,
            },
        }
    }

    /// Sets [`CompatProcessor::incompat_flags`].
    pub fn incompat_flags(mut self, incompat_flags: IncompatFlags) -> Self {
        self.inner.incompat_flags = Some(incompat_flags);
        self
    }

    /// Sets [`CompatProcessor::compat_flags`].
    pub fn compat_flags(mut self, compat_flags: CompatFlags) -> Self {
        self.inner.compat_flags = Some(compat_flags);
        self
    }

    /// Sets [`CompatProcessor::incoming`] strategy.
    ///
    /// Default value is [`CompatStrategy::Reject`].
    pub fn incoming(mut self, strategy: CompatStrategy) -> Self {
        self.inner.incoming = strategy;
        self
    }

    /// Sets [`CompatProcessor::outgoing`] strategy.
    ///
    /// Default value is [`CompatStrategy::Enforce`].
    pub fn outgoing(mut self, strategy: CompatStrategy) -> Self {
        self.inner.outgoing = strategy;
        self
    }

    /// Sets [`CompatProcessor::ignore_signature`].
    ///
    /// Default value is `true`.
    pub fn ignore_signature(mut self, value: bool) -> Self {
        self.inner.ignore_signature = value;
        self
    }

    /// Builds [`CompatProcessor`].
    pub fn build(self) -> CompatProcessor {
        self.inner
    }
}

impl Default for CompatProcessorBuilder {
    fn default() -> Self {
        CompatProcessorBuilder::new()
    }
}

impl IntoCompatProcessor for CompatProcessorBuilder {
    /// Builds [`CompatProcessor`] from [`CompatProcessorBuilder`].
    fn into_compat_processor(self) -> CompatProcessor {
        self.build()
    }
}
