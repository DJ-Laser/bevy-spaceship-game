use bevy::prelude::*;

pub(super) mod engines;
pub(super) mod power;
pub(super) mod weapons;

pub use engines::{ForwardEngine, ReverseEngine, TurningEngine};
pub use power::{PowerSource, PowerStorage};
pub use weapons::{ProjectileWeapon, ShipHull};

/// A marker component indicating that this part is inactive and should not participate in the ship's update cycle.
#[derive(Component, Debug, Default, Clone)]
pub struct InactivePart;
