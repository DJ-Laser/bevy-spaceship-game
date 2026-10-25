use avian2d::prelude::*;
use bevy::{ecs::template::EntityTemplate, prelude::*};

use crate::ship::{
    Spaceship,
    part_components::{InactivePart, PowerStorage, ShipHull},
};

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (energy_hud_text, velocity_hud_text, hull_durability_hud_text),
        );
    }
}

pub fn player_hud(player_ship: EntityTemplate) -> impl Scene {
    bsn! {
      #Hud
      Node {
          position_type: PositionType::Absolute,
          flex_direction: FlexDirection::Column,
          bottom: px(5),
          left: px(15),
      }
      Children [
          EnergyHud(player_ship)
          --
          VelocityHud(player_ship)
          --
          HullDurabilityHud(player_ship)
      ]
    }
}

#[derive(Component, FromTemplate, Debug, Clone)]
#[require(Text)]
struct EnergyHud(Entity);

#[derive(Component, FromTemplate, Debug, Clone)]
#[require(Text)]
struct VelocityHud(Entity);

#[derive(Component, FromTemplate, Debug, Clone)]
#[require(Text)]
struct HullDurabilityHud(Entity);

fn energy_hud_text(
    hud: Single<(&EnergyHud, &mut Text)>,
    spaceships: Query<&Children, With<Spaceship>>,
    energy_storages: Query<&PowerStorage, Without<InactivePart>>,
) {
    let (ship_id, mut text) = hud.into_inner();
    let mut total_capacity = 0.0;
    let mut total_stored = 0.0;

    let Ok(children) = spaceships.get(ship_id.0) else {
        return;
    };

    energy_storages
        .iter_many(children)
        .matched()
        .for_each(|storage| {
            total_capacity += storage.storage_capacity;
            total_stored += storage.stored_energy;
        });

    let percentage = (total_stored / total_capacity) * 100.0;
    text.0 = format!("Energy: {percentage:.0}%");
}

fn velocity_hud_text(
    hud: Single<(&VelocityHud, &mut Text)>,
    ships: Query<(Entity, &LinearVelocity, &AngularVelocity), Without<InactivePart>>,
) {
    let (ship_id, mut text) = hud.into_inner();

    let Some((_, lin_vel, ang_vel)) = ships.iter().find(|(ship, _, _)| *ship == ship_id.0) else {
        return;
    };

    let linear = lin_vel.length();
    let angular = ang_vel.abs();
    text.0 = format!("Velocity: {linear:.2}\nAngular: {angular:.2}");
}

fn hull_durability_hud_text(
    hud: Single<(&HullDurabilityHud, &mut Text)>,
    spaceships: Query<(&Children, Option<&ShipHull>), With<Spaceship>>,
    hulls: Query<&ShipHull, Without<InactivePart>>,
) {
    let (ship_id, mut text) = hud.into_inner();
    let mut total_max_hp = 0.0;
    let mut total_hp = 0.0;

    let Ok((children, hull)) = spaceships.get(ship_id.0) else {
        return;
    };

    if let Some(hull) = hull {
        total_max_hp += hull.max_hp;
        total_hp += hull.current_hp;
    }

    hulls.iter_many(children).matched().for_each(|hull| {
        total_max_hp += hull.max_hp;
        total_hp += hull.current_hp;
    });

    let percentage = (total_hp / total_max_hp) * 100.0;
    text.0 = format!("Hull Durability: {percentage:.0}%");
}
