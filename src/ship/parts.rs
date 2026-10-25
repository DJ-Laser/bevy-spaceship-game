use crate::ship::part_components::*;
use bevy::prelude::*;

pub fn reactor_mk5() -> impl Scene {
    //TODO: Add ways for parts to be "internal", that is, they need a part of the hull to live inside of
    bsn! {
      Name("Reactor MK5")
      PowerSource::new(600.0)
    }
}

pub fn battery_100h4() -> impl Scene {
    bsn! {
      Name("Battery 100H4")
      PowerStorage::new(1000.0)
    }
}

pub fn plasma_cannon() -> impl Scene {
    bsn! {
        Name("Plasma Cannon")
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
        ForwardEngine {
            forward_thrust: 500000.0,
            power_draw: 700.0
        }
    }
}

pub fn engine_rh_500() -> impl Scene {
    bsn! {
        Name("Engine RH-500")
        ReverseEngine {
            reverse_thrust: 100000.0,
            power_draw: 200.0
        }
    }
}

pub fn rcs_tl_150() -> impl Scene {
    bsn! {
        Name("RCS TL-150")
        TurningEngine {
            turn_torque: 1000000000.0,
            power_draw: 50.0
        }
    }
}
