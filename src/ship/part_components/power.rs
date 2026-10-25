use bevy::prelude::*;

use crate::ship::{Spaceship, part_components::InactivePart};

#[derive(Component, Debug, Default, Clone)]
pub struct PowerSource {
    /// The maximum amount of power this part can produce.
    pub generation_capacity: f32,
}

impl PowerSource {
    pub fn new(generation_capacity: f32) -> Self {
        Self {
            generation_capacity,
        }
    }
}

#[derive(Component, Debug, Default, Clone)]
pub struct PowerStorage {
    /// The total amount of energy (power * seconds) this part can store.
    pub storage_capacity: f32,
    /// The ammount of energy (power * seconds) that is currently stoted in this part.
    pub stored_energy: f32,
}

impl PowerStorage {
    pub fn new(storage_capacity: f32) -> Self {
        Self {
            storage_capacity,
            stored_energy: storage_capacity,
        }
    }
}

#[derive(Component, Debug, Default, Clone)]
pub struct PowerConsumer {
    total_requested_power: f32,
    allocated_power: f32,
}

impl PowerConsumer {
    /// Resets the power demands for this part
    fn clear(&mut self) {
        self.total_requested_power = 0.0;
        self.allocated_power = 0.0;
    }

    /// Request power for a component of a part
    pub fn request_power(&mut self, requested_power: f32) {
        self.total_requested_power += requested_power;
    }

    /// Get the ratio of power allocated to power requested
    pub fn allocated_fraction(&self) -> f32 {
        if self.total_requested_power < f32::EPSILON {
            return 1.0;
        }

        self.allocated_power / self.total_requested_power
    }
}

pub fn clear_power_demands(mut consumers: Query<&mut PowerConsumer, Without<InactivePart>>) {
    consumers.iter_mut().for_each(|mut consumer| {
        consumer.clear();
    });
}

pub fn allocate_ship_power(
    time: Res<Time>,
    spaceships: Query<&Children, With<Spaceship>>,
    sources: Query<&PowerSource, Without<InactivePart>>,
    mut storages: Query<&mut PowerStorage, Without<InactivePart>>,
    mut consumers: Query<&mut PowerConsumer, Without<InactivePart>>,
) {
    spaceships.iter().for_each(|children| {
        let total_generation: f32 = sources
            .iter_many(children)
            .matched()
            .map(|source| source.generation_capacity)
            .sum();

        let total_requested_consumption: f32 = consumers
            .iter_many(children)
            .matched()
            .map(|consumer| consumer.total_requested_power)
            .sum();

        // Energy (power per second) surplus/deficit to be absorbed by the power storage parts
        let mut energy_delta = (total_generation - total_requested_consumption) * time.delta_secs();

        let mut iter = storages.iter_many_mut(children).matched();
        while let Some(mut storage) = iter.fetch_next() {
            let min_energy_change = -storage.stored_energy;
            let max_energy_change = storage.storage_capacity - storage.stored_energy;
            let energy_change = energy_delta.clamp(min_energy_change, max_energy_change);

            storage.stored_energy += energy_change;
            energy_delta -= energy_change;

            // Quit early if all delta can be absorbed
            if energy_delta.abs() < f32::EPSILON {
                energy_delta = 0.0;
                break;
            }
        }

        // If excess energy is left over, it is simply wasted.
        let allocated_fraction = if energy_delta >= 0.0 {
            1.0
        } else {
            (total_generation + energy_delta) / total_requested_consumption
        };

        //TODO: Properly allocate reserve power to priority parts
        let mut iter = consumers.iter_many_mut(children).matched();
        while let Some(mut consumer) = iter.fetch_next() {
            consumer.allocated_power = consumer.total_requested_power * allocated_fraction;
        }
    });
}
