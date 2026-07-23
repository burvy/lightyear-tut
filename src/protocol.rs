use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use bevy::{ecs::entity::MapEntities, prelude::*};
use lightyear::prelude::*;
use serde::{Deserialize, Serialize};

pub const TIMESTEP: f64 = 1.0 / 64.0; // 64 is a nice number for computers

pub const SERVER_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 5000);

/// The `ProtocolPlugin` is shared between server and client and helps keeps things in sync
/// especially since this is a server-authoritative setup
pub struct ProtocolPlugin;

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        app.component::<PlayerMarker>().replicate();
        app.component::<PlayerPosition>()
            .replicate()
            .predict()
            .add_linear_interpolation();

        app.add_plugins(input::native::InputPlugin::<Inputs>::default());
    }
}

// lots of components required for replicating things
#[derive(Component, Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct PlayerMarker;

#[derive(Component, Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct PlayerPosition(pub Vec3);

impl Ease for PlayerPosition {
    /// easing copied from simple_box protocol.rs line 45
    fn interpolating_curve_unbounded(start: Self, end: Self) -> impl Curve<Self> {
        FunctionCurve::new(Interval::UNIT, move |t| {
            PlayerPosition(Vec3::lerp(start.0, end.0, t))
        })
    }
}
/// A bunch of inputs that are sent over to the server
/// and must be simulated in one tick.
/// Can contain anything you want
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Reflect, Default)]
pub struct Inputs {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

impl MapEntities for Inputs {
    fn map_entities<M: EntityMapper>(&mut self, _: &mut M) {}
}
