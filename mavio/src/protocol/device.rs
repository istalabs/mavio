use core::marker::PhantomData;

use crate::protocol::{ComponentId, Sequence, Sequencer, SystemId, Versioned, V1, V2};

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
pub struct Device<V: Versioned> {
    pub(super) id: MavLinkId,
    pub(super) sequencer: Sequencer,
    _version: PhantomData<V>,
}

impl MavLinkId {
    /// Creates a new `ID` from the combination of MAVLink system and component ids.
    pub fn new(system: SystemId, component: ComponentId) -> Self {
        Self { system, component }
    }
}

impl Device<V2> {
    /// Creates a new device with specified [`MavLinkId`].
    pub fn new<V: Versioned>(id: MavLinkId) -> Device<V> {
        Device {
            id,
            sequencer: Sequencer::new(),
            _version: PhantomData,
        }
    }

    /// Creates a MAVLink1 device with specified [`MavLinkId`].
    pub fn v1(id: MavLinkId) -> Device<V1> {
        Device::new::<V1>(id)
    }

    /// Creates a MAVLink2 device with specified [`MavLinkId`].
    pub fn v2(id: MavLinkId) -> Self {
        Self::new(id)
    }
}

impl<V: Versioned> Device<V> {
    /// Device `ID`.
    #[inline(always)]
    pub fn id(&self) -> MavLinkId {
        self.id
    }

    /// MAVlink system `ID`.
    pub fn system_id(&self) -> SystemId {
        self.id.system
    }

    /// MAVlink component `ID`.
    pub fn component_id(&self) -> ComponentId {
        self.id.component
    }

    /// Next MAVLink frame sequence.
    pub fn next_sequence(&self) -> Sequence {
        self.sequencer.next()
    }

    /// Returns a reference to internal [`Sequencer`].
    pub fn sequencer(&self) -> &Sequencer {
        &self.sequencer
    }
}
