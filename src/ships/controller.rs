use bevy::{ecs::entity::EntityEquivalent, prelude::*};

use crate::ships::{ForwardEngine, InactiveSystem, ReverseEngine, TurningEngine};

pub mod dumb_fire;
pub mod player;

struct EngineTotals {
    forward_thrust: f32,
    reverse_thrust: f32,
    turn_torque: f32,
}

impl EngineTotals {
    fn from_query<EntityList: IntoIterator<Item: EntityEquivalent>>(
        children: EntityList,
        engines: Query<
            AnyOf<(&ForwardEngine, &ReverseEngine, &TurningEngine)>,
            Without<InactiveSystem>,
        >,
    ) -> Self {
        let mut totals = Self {
            forward_thrust: 0.0,
            reverse_thrust: 0.0,
            turn_torque: 0.0,
        };

        engines.iter_many(children).matched().for_each(
            |(forward_engine, reverse_engine, turn_engine)| {
                if let Some(forward_engine) = forward_engine {
                    totals.forward_thrust += forward_engine.forward_thrust;
                }

                if let Some(reverse_engine) = reverse_engine {
                    totals.reverse_thrust += reverse_engine.reverse_thrust;
                }

                if let Some(turn_engine) = turn_engine {
                    totals.turn_torque += turn_engine.turn_torque;
                }
            },
        );

        totals
    }
}
