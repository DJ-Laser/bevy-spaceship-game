use avian2d::{dynamics::rigid_body::forces::ForcesItem, prelude::*};
use bevy::{ecs::relationship::Relationship as _, prelude::*};
use leafwing_input_manager::action_state::ActionState;

use crate::input::PlayerAction;

use super::{InactiveSystem, PowerConsumer, Spaceship};

pub trait MovementComponent: Component {
    fn get_throttle(actions: &ActionState<PlayerAction>) -> f32;
    fn get_max_power_draw(&self) -> f32;
    fn apply_movement(&self, throttle: f32, forces: &mut ForcesItem);
}

pub fn request_power<C: MovementComponent>(
    spaceships: Query<(Entity, &ActionState<PlayerAction>), With<Spaceship>>,
    mut components: Query<(&ChildOf, &C, &mut PowerConsumer), Without<InactiveSystem>>,
) {
    spaceships.iter().for_each(|(spaceship_id, actions)| {
        let throttle_abs = C::get_throttle(actions).abs();

        components
            .iter_mut()
            .for_each(|(p, component, mut consumer)| {
                if p.get() != spaceship_id {
                    return;
                }

                consumer.request_power(throttle_abs * component.get_max_power_draw());
            });
    });
}

pub fn apply_movement<C: MovementComponent>(
    mut spaceships: Query<(Entity, &ActionState<PlayerAction>, Forces), With<Spaceship>>,
    components: Query<(&ChildOf, &C, &PowerConsumer), Without<InactiveSystem>>,
) {
    spaceships
        .iter_mut()
        .for_each(|(spaceship_id, actions, mut forces)| {
            let throttle = C::get_throttle(actions);

            components.iter().for_each(|(p, component, consumer)| {
                if p.get() != spaceship_id {
                    return;
                }

                let adjusted_throttle = throttle * consumer.allocated_fraction();
                component.apply_movement(adjusted_throttle, &mut forces);
            });
        });
}

/// Engines that propel the ship forward
#[derive(Component, Debug, Default, Clone)]
#[require(PowerConsumer)]
pub struct ForwardEngine {
    pub engine_force: f32,
    pub power_draw: f32,
}

impl MovementComponent for ForwardEngine {
    fn get_throttle(actions: &ActionState<PlayerAction>) -> f32 {
        actions.clamped_button_value(&PlayerAction::Accelerate)
    }

    fn get_max_power_draw(&self) -> f32 {
        self.power_draw
    }

    fn apply_movement(&self, throttle: f32, forces: &mut ForcesItem) {
        forces.apply_local_force(Vec2::Y * throttle * self.engine_force);
    }
}

/// Engines that propel the ship in reverse
#[derive(Component, Debug, Default, Clone)]
#[require(PowerConsumer)]
pub struct ReverseEngine {
    pub engine_force: f32,
    pub power_draw: f32,
}

impl MovementComponent for ReverseEngine {
    fn get_throttle(actions: &ActionState<PlayerAction>) -> f32 {
        actions.clamped_button_value(&PlayerAction::Reverse)
    }

    fn get_max_power_draw(&self) -> f32 {
        self.power_draw
    }

    fn apply_movement(&self, throttle: f32, forces: &mut ForcesItem) {
        forces.apply_local_force(Vec2::NEG_Y * throttle * self.engine_force);
    }
}

/// Engines that change the ship's rotation
#[derive(Component, Debug, Default, Clone)]
#[require(PowerConsumer)]
pub struct TurningEngine {
    pub turn_torque: f32,
    pub power_draw: f32,
}

impl MovementComponent for TurningEngine {
    fn get_throttle(actions: &ActionState<PlayerAction>) -> f32 {
        -actions.clamped_value(&PlayerAction::Turn)
    }

    fn get_max_power_draw(&self) -> f32 {
        self.power_draw
    }

    fn apply_movement(&self, throttle: f32, forces: &mut ForcesItem) {
        forces.apply_torque(throttle * self.turn_torque);
    }
}

/// Systems that limit the ship's linear and rotational velocity
#[derive(Component, Debug, Default, Clone)]
#[require(PowerConsumer)]
pub struct InertialDampener {
    pub linear_damping_constant: f32,
    pub angular_damping_constant: f32,
    pub power_draw: f32,
}

impl MovementComponent for InertialDampener {
    fn get_throttle(_actions: &ActionState<PlayerAction>) -> f32 {
        1.0
    }

    fn get_max_power_draw(&self) -> f32 {
        self.power_draw
    }

    fn apply_movement(&self, throttle: f32, forces: &mut ForcesItem) {
        forces.apply_force(-forces.linear_velocity() * throttle * self.linear_damping_constant);
        forces.apply_torque(-forces.angular_velocity() * throttle * self.angular_damping_constant);
    }
}
