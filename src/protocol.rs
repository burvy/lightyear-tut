use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub const TIMESTEP: f64 = 1.0 / 64.0

pub struct ProtocolPlugin;

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        println!("Protocol Plugin added!"); // TODO: Remove debug logging
    }
}

#[derive(Component, Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct PlayerMarker;
/// A bunch of inputs that are sent over to the server
/// and must be simulated in one tick.
/// Can contain anything you want
pub struct Inputs {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}
