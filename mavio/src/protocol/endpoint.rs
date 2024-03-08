use core::marker::PhantomData;

use crate::protocol::{Behold, ComponentId, Sequence, Sequencer, SystemId};

use crate::prelude::*;

/// MAVLink device `ID`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MavLinkId {
    /// System `ID`.
    pub system: SystemId,
    /// Component `ID`.
    pub component: ComponentId,
}

/// MAVLink device with defined `ID` and internal frame sequence counter.
///
/// # Examples
///
/// ```no_run
/// use mavio::dialects::minimal::messages::Heartbeat;
/// use mavio::prelude::*;
///
/// // Create a `MAVLink2` device with system and component ids
/// let device = Endpoint::v2(MavLinkId::new(17, 42));
/// device.advance(3).discard();
///
/// // Build a new frame from the provided message
/// let frame = device.next_frame(&Heartbeat::default()).unwrap();
///
/// assert_eq!(frame.sequence(), 3, "should be correct sequence number");
/// assert_eq!(frame.system_id(), 17, "should be the defined system `ID`");
/// assert_eq!(frame.component_id(), 42, "should be the defined component `ID`");
/// ```
#[derive(Debug)]
pub struct Endpoint<V: MaybeVersioned> {
    id: MavLinkId,
    sequencer: Sequencer,
    _version: PhantomData<V>,
}

impl MavLinkId {
    /// Creates a new `ID` from the combination of MAVLink system and component ids.
    pub fn new(system: SystemId, component: ComponentId) -> Self {
        Self { system, component }
    }
}

impl Endpoint<Versionless> {
    /// Creates a new device with specified [`MavLinkId`].
    pub fn new<V: MaybeVersioned>(id: MavLinkId) -> Endpoint<V> {
        Endpoint {
            id,
            sequencer: Sequencer::new(),
            _version: PhantomData,
        }
    }

    /// Creates a MAVLink1 device with specified [`MavLinkId`].
    #[inline]
    pub fn v1(id: MavLinkId) -> Endpoint<V1> {
        Endpoint::new(id)
    }

    /// Creates a MAVLink2 device with specified [`MavLinkId`].
    #[inline]
    pub fn v2(id: MavLinkId) -> Endpoint<V2> {
        Endpoint::new(id)
    }

    /// Creates a device without a specified MAVLink protocol version.
    #[inline]
    pub fn versionless(id: MavLinkId) -> Endpoint<Versionless> {
        Endpoint::new(id)
    }

    /// Produces a next versionless frame from MAVLink message.
    ///
    /// The actual protocol version still has to be specified as a generic parameter using
    /// [turbofish](https://turbo.fish/about) syntax.
    pub fn next_frame<V: Versioned>(&self, message: &dyn Message) -> Result<Frame<Versionless>> {
        Ok(self._next_frame::<V>(message)?.into_versionless())
    }
}

impl<V: MaybeVersioned> Endpoint<V> {
    /// Device `ID`.
    #[inline(always)]
    pub fn id(&self) -> MavLinkId {
        self.id
    }

    /// MAVLink system `ID`.
    #[inline(always)]
    pub fn system_id(&self) -> SystemId {
        self.id.system
    }

    /// MAVLink component `ID`.
    #[inline(always)]
    pub fn component_id(&self) -> ComponentId {
        self.id.component
    }

    /// Next MAVLink frame sequence.
    #[inline(always)]
    pub fn next_sequence(&self) -> Sequence {
        self.sequencer.next()
    }

    /// Returns a reference to internal [`Sequencer`].
    #[inline(always)]
    pub fn sequencer(&self) -> &Sequencer {
        &self.sequencer
    }

    /// Skips `increment` items in sequence and return the updated current value.
    ///
    /// The return value is wrapped in [`Behold`] since it is not guaranteed in multithreaded
    /// environments, that the [`Endpoint::next_frame`] will use the same value of a sequence in
    /// this thread.
    pub fn advance(&self, increment: Sequence) -> Behold<Sequence> {
        self.sequencer.advance(increment)
    }

    /// Forks existing endpoint.
    ///
    /// Forking is similar to cloning, except the internal frame [`Sequencer`] will be forked to
    /// start from the next value. This method is available for all targets, while cloning is
    /// possible only for `alloc` targets.
    ///
    /// See [`Sequencer::fork`] for details.
    pub fn fork(&self) -> Self {
        Self {
            id: self.id,
            sequencer: self.sequencer.fork(),
            _version: PhantomData,
        }
    }

    /// Synchronizes this endpoint with another one.
    ///
    /// Synchronizes internal sequencer with the sequencer of the `other` [`Endpoint`].
    ///
    /// See [`Sequencer::sync`] for details.
    pub fn sync<Version: MaybeVersioned>(&self, other: &Endpoint<Version>) {
        self.sequencer.sync(other.sequencer())
    }

    /// <sup>`alloc`</sup>
    /// Joins another endpoint with current one.
    ///
    /// From this moment internal sequencers will share the same counter. The current sequencer will
    /// be synced with the one it joins.
    ///
    /// Available only when `alloc` feature is enabled.
    ///
    /// See [`Sequencer::join`] for details.
    #[cfg(feature = "alloc")]
    pub fn join<Version: MaybeVersioned>(&self, other: &mut Endpoint<Version>) {
        self.sequencer.join(&mut other.sequencer)
    }

    fn _next_frame<Version: Versioned>(&self, message: &dyn Message) -> Result<Frame<Version>> {
        let frame = Frame::builder()
            .sequence(self.next_sequence())
            .system_id(self.system_id())
            .component_id(self.component_id())
            .version(Version::v())
            .message(message)?
            .build();
        Ok(frame)
    }
}

impl<V: Versioned> Endpoint<V> {
    /// Produces a next frame from MAVLink message.
    pub fn next_frame(&self, message: &dyn Message) -> Result<Frame<V>> {
        self._next_frame::<V>(message)
    }
}

#[cfg(feature = "alloc")]
impl<V: MaybeVersioned> Clone for Endpoint<V> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            sequencer: self.sequencer.clone(),
            _version: PhantomData,
        }
    }
}
