use super::{InactiveSystem, Spaceship};
use avian2d::parry::utils::hashmap::HashMap;
use bevy::{
    ecs::{component::ComponentId, relationship::Relationship as _},
    prelude::*,
};

#[derive(Component, Debug, Default, Clone)]
pub struct PowerSource {
    /// The maximum amount of power this system can produce.
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
    /// The total amount of energy (power * seconds) this system can store.
    pub storage_capacity: f32,
    /// The ammount of energy (power * seconds) that is currently stoted in this system.
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
    component_requested_power: HashMap<ComponentId, f32>,
    total_requested_power: f32,
    allocated_power: f32,
}

impl PowerConsumer {
    /// Resets the power demands for this system
    pub fn clear(&mut self) {
        self.component_requested_power.clear();
        self.total_requested_power = 0.0;
        self.allocated_power = 0.0;
    }

    /// Get the amount of power this system needs to function at full capacity.
    pub fn requested_power(&self) -> f32 {
        self.total_requested_power
    }

    /// Set the amount of power available for this system to use.
    pub fn set_allocated_power(&mut self, allocated_power: f32) {
        self.allocated_power = allocated_power;
    }

    /// Request power for a component of a system
    pub fn request_power(&mut self, id: ComponentId, requested_power: f32) {
        *self.component_requested_power.entry(id).or_insert(0.0) += requested_power;
        self.total_requested_power += requested_power;
    }

    /// Get the alloacted power for a component of a system
    pub fn allocated_power(&self, id: ComponentId) -> f32 {
        if self.total_requested_power < f32::EPSILON {
            return 0.0;
        }

        let requested_power = self
            .component_requested_power
            .get(&id)
            .copied()
            .unwrap_or(0.0);

        (requested_power / self.total_requested_power) * self.allocated_power
    }
}

pub fn clear_power_demands(mut consumers: Query<&mut PowerConsumer, Without<InactiveSystem>>) {
    consumers.iter_mut().for_each(|mut consumer| {
        consumer.clear();
    });
}

pub fn allocate_ship_power(
    time: Res<Time>,
    spaceships: Query<Entity, With<Spaceship>>,
    sources: Query<(&ChildOf, &PowerSource), Without<InactiveSystem>>,
    mut storages: Query<(&ChildOf, &mut PowerStorage), Without<InactiveSystem>>,
    mut consumers: Query<(&ChildOf, &mut PowerConsumer), Without<InactiveSystem>>,
) {
    spaceships.iter().for_each(|spaceship_id| {
        let total_generation: f32 = sources
            .iter()
            .filter(|(p, _)| p.get() == spaceship_id)
            .map(|(_, source)| source.generation_capacity)
            .sum();

        let total_requested_consumption: f32 = consumers
            .iter()
            .filter(|(p, _)| p.get() == spaceship_id)
            .map(|(_, consumer)| consumer.requested_power())
            .sum();

        // Energy (power per second) surplus/deficit to be absorbed by the power storage systems
        let mut energy_delta = (total_generation - total_requested_consumption) * time.delta_secs();

        for (p, mut storage) in storages.iter_mut() {
            if p.get() != spaceship_id {
                continue;
            }

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
        if energy_delta >= 0.0 {
            consumers
                .iter_mut()
                .filter(|(p, _)| p.get() == spaceship_id)
                .for_each(|(_, mut consumer)| {
                    let requested_power = consumer.requested_power();
                    consumer.set_allocated_power(requested_power);
                });

            return;
        }

        //TODO: Properly allocate reserve power to priority systems
        consumers
            .iter_mut()
            .filter(|(p, _)| p.get() == spaceship_id)
            .for_each(|(_, mut consumer)| {
                consumer.allocated_power = 0.0;
            });
    });
}
