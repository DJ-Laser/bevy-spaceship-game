use avian2d::{dynamics::rigid_body::forces::ForcesItem, prelude::*};
use bevy::{ecs::relationship::Relationship as _, prelude::*};
use leafwing_input_manager::{Actionlike, action_state::ActionState};

use crate::ships::{InactiveSystem, PowerConsumer};

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
#[actionlike(Axis)]
pub enum EngineAction {
    Thrust,
    Turn,
}

pub trait EngineComponent: Component {
    fn get_throttle(actions: &ActionState<EngineAction>) -> f32;
    fn get_max_power_draw(&self) -> f32;
    fn apply_movement(&self, throttle: f32, forces: &mut ForcesItem);
}

pub fn request_engine_power<C: EngineComponent>(
    mut engines: Query<
        (&C, &ActionState<EngineAction>, &mut PowerConsumer),
        Without<InactiveSystem>,
    >,
) {
    engines
        .iter_mut()
        .for_each(|(component, actions, mut consumer)| {
            let throttle_abs = C::get_throttle(actions).abs();

            consumer.request_power(throttle_abs * component.get_max_power_draw());
        });
}

pub fn fire_engine<C: EngineComponent>(
    mut spaceships: Query<Forces>,
    engines: Query<
        (&ChildOf, &C, &ActionState<EngineAction>, &PowerConsumer),
        Without<InactiveSystem>,
    >,
) {
    engines
        .iter()
        .for_each(|(p, component, actions, consumer)| {
            let Ok(mut forces) = spaceships.get_mut(p.get()) else {
                return;
            };

            let throttle = C::get_throttle(actions);

            let adjusted_throttle = throttle * consumer.allocated_fraction();
            component.apply_movement(adjusted_throttle, &mut forces);
        });
}

/// Engines that propel the ship forward
#[derive(Component, Debug, Default, Clone)]
#[require(PowerConsumer, ActionState<EngineAction>)]
pub struct ForwardEngine {
    pub forward_thrust: f32,
    pub power_draw: f32,
}

impl EngineComponent for ForwardEngine {
    fn get_throttle(actions: &ActionState<EngineAction>) -> f32 {
        actions.clamped_value(&EngineAction::Thrust).max(0.0)
    }

    fn get_max_power_draw(&self) -> f32 {
        self.power_draw
    }

    fn apply_movement(&self, throttle: f32, forces: &mut ForcesItem) {
        forces.apply_local_force(Vec2::Y * throttle * self.forward_thrust);
    }
}

/// Engines that propel the ship in reverse
#[derive(Component, Debug, Default, Clone)]
#[require(PowerConsumer, ActionState<EngineAction>)]
pub struct ReverseEngine {
    pub reverse_thrust: f32,
    pub power_draw: f32,
}

impl EngineComponent for ReverseEngine {
    fn get_throttle(actions: &ActionState<EngineAction>) -> f32 {
        actions.clamped_value(&EngineAction::Thrust).min(0.0)
    }

    fn get_max_power_draw(&self) -> f32 {
        self.power_draw
    }

    fn apply_movement(&self, throttle: f32, forces: &mut ForcesItem) {
        forces.apply_local_force(Vec2::Y * throttle * self.reverse_thrust);
    }
}

/// Engines that change the ship's rotation
#[derive(Component, Debug, Default, Clone)]
#[require(PowerConsumer, ActionState<EngineAction>)]
pub struct TurningEngine {
    pub turn_torque: f32,
    pub power_draw: f32,
}

impl EngineComponent for TurningEngine {
    fn get_throttle(actions: &ActionState<EngineAction>) -> f32 {
        actions.clamped_value(&EngineAction::Turn)
    }

    fn get_max_power_draw(&self) -> f32 {
        self.power_draw
    }

    fn apply_movement(&self, throttle: f32, forces: &mut ForcesItem) {
        forces.apply_torque(throttle * self.turn_torque);
    }
}
