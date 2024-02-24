use core::marker::PhantomData;

use crate::protocol::{ComponentId, Sequence, Sequencer, SystemId, Versioned, V1, V2};

/// MAVLink device `ID`.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeviceId {
    /// System `ID`.
    pub system: SystemId,
    /// Component `ID`.
    pub component: ComponentId,
}

/// MAVLink device.
pub struct Device<V: Versioned> {
    id: DeviceId,
    sequencer: Sequencer,
    _version: PhantomData<V>,
}

impl Device<V2> {
    /// Creates a new device with specified [`DeviceId`].
    pub fn new<V: Versioned>(id: DeviceId) -> Device<V> {
        Device {
            id,
            sequencer: Sequencer::new(),
            _version: PhantomData,
        }
    }

    /// Creates a MAVLink1 device with specified [`DeviceId`].
    pub fn v1(id: DeviceId) -> Device<V1> {
        Device::new::<V1>(id)
    }

    /// Creates a MAVLink2 device with specified [`DeviceId`].
    pub fn v2(id: DeviceId) -> Self {
        Self::new(id)
    }

    /// Device `ID`.
    #[inline(always)]
    pub fn id(&self) -> DeviceId {
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
