use crate::ship::part_components::*;
use bevy::{
    color::palettes::css::{BLUE, GRAY, GREEN, PURPLE, RED, YELLOW},
    prelude::*,
};

pub mod lists;

#[derive(Component, Default, Debug, Clone)]
pub enum PartCategory {
    Weapons,
    Engines,
    Power,
    Defense,
    Utility,
    #[default]
    Miscellaneous,
}

#[derive(Component, Clone, Default)]
pub struct Cost(pub i64);

pub fn reactor_mk5() -> impl Scene {
    //TODO: Add ways for parts to be "internal", that is, they need a part of the hull to live inside of
    bsn! {
      Name("Reactor MK5")
      Cost(15_000)
      Mesh2d(asset_value(Rectangle::new(50.0, 50.0)))
      MeshMaterial2d<ColorMaterial>(asset_value(Into::<Color>::into(GREEN)))
      PartCategory::Power
      PowerSource::new(600.0)
    }
}

pub fn battery_100h4() -> impl Scene {
    bsn! {
      Name("Battery 100H4")
      Cost(2_000)
      Mesh2d(asset_value(Rectangle::new(25.0, 50.0)))
      MeshMaterial2d<ColorMaterial>(asset_value(Into::<Color>::into(YELLOW)))
      PartCategory::Power
      PowerStorage::new(1000.0)
    }
}

pub fn plasma_cannon() -> impl Scene {
    bsn! {
        Name("Plasma Cannon")
        Cost(1_000)
        Mesh2d(asset_value(Rectangle::new(15.0, 100.0)))
        MeshMaterial2d<ColorMaterial>(asset_value(Into::<Color>::into(GRAY)))
        PartCategory::Weapons
        ProjectileWeapon {
            velocity: 500.0,
            frequency: 4.0,
            power_draw: 100.0,
        }
    }
}

pub fn engine_xn_2000() -> impl Scene {
    bsn! {
        Name("Engine XN-2000")
        Cost(20_000)
        Mesh2d(asset_value(Rectangle::new(35.0, 25.0)))
        MeshMaterial2d<ColorMaterial>(asset_value(Into::<Color>::into(BLUE)))
        PartCategory::Engines
        ForwardEngine {
            forward_thrust: 500000.0,
            power_draw: 700.0
        }
    }
}

pub fn engine_rh_500() -> impl Scene {
    bsn! {
        Name("Reverse Engine RH-500")
        Cost(5_000)
        Mesh2d(asset_value(Rectangle::new(35.0, 25.0)))
        MeshMaterial2d<ColorMaterial>(asset_value(Into::<Color>::into(RED)))
        PartCategory::Engines
        ReverseEngine {
            reverse_thrust: 100000.0,
            power_draw: 200.0
        }
    }
}

pub fn rcs_tl_150() -> impl Scene {
    bsn! {
        Name("RCS Turning TL-150")
        Cost(10_000)
        Mesh2d(asset_value(Rectangle::new(25.0, 35.0)))
        MeshMaterial2d<ColorMaterial>(asset_value(Into::<Color>::into(PURPLE)))
        PartCategory::Engines
        TurningEngine {
            turn_torque: 1000000000.0,
            power_draw: 50.0
        }
    }
}
