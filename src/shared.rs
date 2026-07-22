use bevy::prelude::*;

use crate::protocol::{Inputs, PlayerPosition, TIMESTEP};

const SPEED: f32 = 5.0;

/// translates inputs into a movement vector
pub fn apply_input(pos: &mut PlayerPosition, input: &Inputs) {
    let mut dir = Vec3::ZERO;
    if input.up {
        dir.z -= 1.0;
    }
    if input.down {
        dir.z += 1.0;
    }
    if input.left {
        dir.x -= 1.0;
    }
    if input.right {
        dir.x += 1.0;
    }
    pos.0 += dir.normalize_or_zero() * SPEED * TIMESTEP as f32;
}
