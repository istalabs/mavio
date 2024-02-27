use core::fmt::{Debug, Formatter};
use core::sync::atomic::Ordering;

use crate::protocol::{Sequence, Unsafe};

#[cfg(not(feature = "alloc"))]
use no_std::_Sequencer;
#[cfg(feature = "alloc")]
use standard::_Sequencer;

/// Converts value into a [`Sequencer`].
///
/// This trait is implemented for [`Sequencer`] and [`Sequence`]. For the latter it creates a new
/// sequencer initialized with sequence value.
///
/// This trait also implemented for [`Sequencer`] passed by reference but behavior is different for
/// `alloc` and `no_alloc` targets:
///
/// * `alloc`: clones a sequencer
/// * `no_alloc`: forks a sequencer using [`Sequencer::fork`]
pub trait IntoSequencer {
    /// Converts value into a [`Sequencer`].
    fn into_sequencer(self) -> Sequencer;
}

/// Incremental MAVLink frame sequence.
///
/// # Examples
///
/// ```rust
/// use mavio::protocol::Sequencer;
///
/// // Start a new sequence
/// let seq = Sequencer::new();
/// assert_eq!(seq.next(), 0, "initial value");
/// assert_eq!(seq.next(), 1, "should increment");
///
/// // For original sequence to an independent counter
/// let forked = seq.fork();
/// assert_eq!(forked.next(), 2, "should increment");
/// assert_eq!(forked.next(), 3, "should increment");
/// assert_eq!(seq.current(), 2, "should be independent");
///
/// // Synchronize original sequence with the forked one
/// seq.sync(&forked);
/// assert_eq!(seq.next(), 4, "should be updated");
/// assert_eq!(forked.next(), 4, "forked sequence is still independent");
/// ```
pub struct Sequencer(_Sequencer);

impl Sequencer {
    /// Default constructor, starts from `0`.
    #[inline(always)]
    pub fn new() -> Self {
        Self::init(0)
    }

    /// Instantiates sequencer initialized with a specific `value`.
    #[inline(always)]
    pub fn init(value: Sequence) -> Self {
        Self(_Sequencer::init_with(value))
    }

    /// Forks sequencer to a new independent sequence counter that starts from the next value.
    ///
    /// # Examples
    ///
    /// ```
    /// use mavio::protocol::Sequencer;
    ///
    /// let seq = Sequencer::new();
    /// let forked = seq.fork();
    ///
    /// assert_eq!(forked.next(), 0, "initial value");
    /// assert_eq!(forked.next(), 1, "should increment");
    /// assert_eq!(seq.current(), 0, "should stay the same");
    /// ```
    #[inline(always)]
    pub fn fork(&self) -> Self {
        Self::init(self.current())
    }

    /// <sup>`alloc`</sup>
    /// Joins another sequencer with current one.
    ///
    /// From this moment both sequencers will share the same counter. The current sequence will be
    /// synced with the one it joins.
    ///
    /// Available only when `alloc` feature is enabled.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[cfg(feature = "alloc")]{
    /// use mavio::protocol::Sequencer;
    ///
    /// let main = Sequencer::new();
    /// let mut forked = main.fork();
    ///
    /// // Proceed with the forked sequence
    /// assert_eq!(forked.next(), 0, "initial value");
    /// assert_eq!(forked.next(), 1, "should increment");
    /// assert_eq!(forked.next(), 2, "should increment");
    ///
    /// main.join(&mut forked);
    /// assert_eq!(main.next(), 3, "should be synced with the joined sequence");
    /// assert_eq!(forked.next(), 4, "joined sequence now shares the same counter");
    /// }
    /// ```
    #[cfg(feature = "alloc")]
    pub fn join(&self, other: &mut Sequencer) {
        self.sync(other);
        other.0 = self.0.clone();
    }

    /// Synchronizes this sequencer with another one.
    #[inline(always)]
    pub fn sync(&self, other: &Sequencer) {
        self.rewind(other.current())
    }

    /// Obtains the next value of the sequence.
    #[inline(always)]
    pub fn next(&self) -> Sequence {
        self.next_value()
    }

    /// Returns the current value without incrementing internal counter.
    #[inline(always)]
    pub fn current(&self) -> Sequence {
        self.0 .0.load(Ordering::Acquire)
    }

    /// Rewinds sequence to a specific value.
    #[inline(always)]
    pub fn rewind(&self, value: Sequence) {
        self.0 .0.store(value, Ordering::Release)
    }

    /// Skips `increment` items in sequence and return the updated current value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mavio::protocol::Sequencer;
    ///
    /// let seq = Sequencer::new();
    /// seq.advance(3).discard();
    /// assert_eq!(seq.next(), 3, "should skip 0, 1, and 2");
    /// ```
    ///
    /// The return value is wrapped with [`Unsafe`] since it is not guaranteed in multithreaded
    /// environments that the [`Sequencer::next`] will return the same value in this thread. Use
    /// [`Unsafe::unwrap`] to explicitly acknowledge that you understand what you are doing and
    /// retrieve the value or discard it by calling [`Unsafe::discard`].
    #[inline(always)]
    #[must_use]
    pub fn advance(&self, increment: Sequence) -> Unsafe<Sequence> {
        Unsafe::new(self.0 .0.fetch_add(increment, Ordering::Release) + increment)
    }

    #[inline]
    fn next_value(&self) -> Sequence {
        self.0 .0.fetch_add(1, Ordering::Release)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Sequencer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8(self.current())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Sequencer {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let value = u8::deserialize(d)?;
        Ok(Sequencer::init(value))
    }
}

impl IntoSequencer for Sequencer {
    /// Passes [`Sequencer`] unchanged.
    #[inline(always)]
    fn into_sequencer(self) -> Sequencer {
        self
    }
}

#[cfg(not(feature = "alloc"))]
impl IntoSequencer for &Sequencer {
    /// Forks a reference to a [`Sequencer`] into a new forked sequencer for `no_alloc` targets.
    #[inline(always)]
    fn into_sequencer(self) -> Sequencer {
        self.fork()
    }
}

#[cfg(feature = "alloc")]
impl IntoSequencer for &Sequencer {
    /// Clones [`Sequencer`] passed by reference for `alloc` targets.
    #[inline(always)]
    fn into_sequencer(self) -> Sequencer {
        self.clone()
    }
}

impl IntoSequencer for Sequence {
    /// Creates a [`Sequencer`] initialized with [`Sequence`] value.
    #[inline]
    fn into_sequencer(self) -> Sequencer {
        Sequencer::init(self)
    }
}

impl From<&Sequencer> for Sequencer {
    fn from(value: &Sequencer) -> Self {
        value.into_sequencer()
    }
}

impl From<Sequence> for Sequencer {
    fn from(value: Sequence) -> Self {
        value.into_sequencer()
    }
}

#[cfg(feature = "alloc")]
impl Clone for Sequencer {
    /// <sup>`alloc`</sup>
    /// Creates a new sequencer which shares the same counter with the current one.
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl Default for Sequencer {
    /// Creates a default [`Sequencer`] starting from `0`.
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for Sequencer {
    type Item = Sequence;

    /// Gets the next element of a sequence. Always returns [`Some`].
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.next_value())
    }
}

impl Debug for Sequencer {
    /// Formats [`Sequencer`] showing its current value.
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Sequencer").field(&self.current()).finish()
    }
}

#[cfg(feature = "alloc")]
mod standard {
    use alloc::sync::Arc;
    use core::sync::atomic::AtomicU8;

    use crate::protocol::Sequence;

    #[derive(Clone)]
    pub struct _Sequencer(pub(super) Arc<AtomicU8>);

    impl _Sequencer {
        #[inline]
        pub(super) fn init_with(value: Sequence) -> Self {
            Self(Arc::new(AtomicU8::new(value)))
        }
    }
}

#[cfg(not(feature = "alloc"))]
mod no_std {
    use core::sync::atomic::AtomicU8;

    use crate::protocol::Sequence;

    pub struct _Sequencer(pub(super) AtomicU8);

    impl _Sequencer {
        #[inline]
        pub(super) fn init_with(value: Sequence) -> Self {
            Self(AtomicU8::new(value))
        }
    }
}
