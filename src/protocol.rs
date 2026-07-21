use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use bevy::prelude::*;
use lightyear::prelude::*;
use serde::{Deserialize, Serialize};

pub const TIMESTEP: f64 = 1.0 / 64.0;

pub const SERVER_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 5000);

pub struct ProtocolPlugin;

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        println!("Protocol Plugin added!"); // TODO: Remove debug logging

        app.component::<PlayerMarker>().replicate();
        app.component::<PlayerPosition>().replicate();
    }
}

// lots of components required for replicating things
#[derive(Component, Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct PlayerMarker;

#[derive(Component, Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct PlayerPosition(pub Vec3);

/// A bunch of inputs that are sent over to the server
/// and must be simulated in one tick.
/// Can contain anything you want
pub struct Inputs {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}
