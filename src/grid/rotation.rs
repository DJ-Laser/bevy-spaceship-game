use std::f32::consts::{FRAC_PI_2, PI};

use bevy::prelude::*;

#[derive(Component, Clone, Copy, Default, PartialEq, Eq)]
pub enum RotationIncrement {
    #[default]
    Rotate0,
    Rotate90,
    Rotate180,
    Rotate270,
}

impl RotationIncrement {
    pub fn rotate_tile_pos(&self, pos: IVec2) -> IVec2 {
        match self {
            RotationIncrement::Rotate0 => pos,
            RotationIncrement::Rotate90 => ivec2(pos.y, -pos.x),
            RotationIncrement::Rotate180 => -pos,
            RotationIncrement::Rotate270 => ivec2(-pos.y, pos.x),
        }
    }

    pub fn rotation_radians(&self) -> f32 {
        match self {
            RotationIncrement::Rotate0 => 0.0,
            RotationIncrement::Rotate90 => -FRAC_PI_2,
            RotationIncrement::Rotate180 => PI,
            RotationIncrement::Rotate270 => FRAC_PI_2,
        }
    }

    pub fn inverse(&self) -> Self {
        use RotationIncrement::*;
        match self {
            RotationIncrement::Rotate0 => Rotate0,
            RotationIncrement::Rotate90 => Rotate270,
            RotationIncrement::Rotate180 => Rotate180,
            RotationIncrement::Rotate270 => Rotate90,
        }
    }
}

impl RotationIncrement {
    pub fn prev(&self, style: &RotationStyle) -> Self {
        style.cycle_increment(*self, true)
    }

    pub fn next(&self, style: &RotationStyle) -> Self {
        style.cycle_increment(*self, false)
    }
}

#[derive(Component, Clone, Default, PartialEq, Eq)]
#[require(RotationIncrement)]
pub enum RotationStyle {
    /// No rotation options
    #[default]
    NoRotation,
    /// 4 way rotation in 90 degree increments
    Rotate90,
    /// 2 way rotation with 0 and 180 degree rotations
    Rotate180,
    /// 2 way rotation with 0 and 90 degree rotations
    Rotate90Symmetric,
}

impl RotationStyle {
    fn rotation_order(&self) -> &'static [RotationIncrement] {
        use RotationIncrement::*;
        match self {
            RotationStyle::NoRotation => &[Rotate0],
            RotationStyle::Rotate90 => &[Rotate0, Rotate90, Rotate180, Rotate270],
            RotationStyle::Rotate180 => &[Rotate0, Rotate180],
            RotationStyle::Rotate90Symmetric => &[Rotate0, Rotate90],
        }
    }

    fn cycle_increment(&self, increment: RotationIncrement, reverse: bool) -> RotationIncrement {
        let order = self.rotation_order();
        let Some(idx) = order.iter().position(|item| *item == increment) else {
            return order[0];
        };

        if reverse {
            if idx == 0 {
                order[order.len() - 1]
            } else {
                order[idx - 1]
            }
        } else {
            let new_idx = idx + 1;
            if new_idx < order.len() {
                order[new_idx]
            } else {
                order[0]
            }
        }
    }
}
