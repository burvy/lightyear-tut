use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use avian3d::{
    dynamics::{
        integrator::Gravity,
        rigid_body::LinearVelocity,
        solver::islands::{IslandPlugin, IslandSleepingPlugin},
    },
    interpolation::PhysicsInterpolationPlugin,
    physics_transform::Position,
    PhysicsPlugins,
};
use bevy::{ecs::entity::MapEntities, prelude::*};
use lightyear::{avian3d::plugin::LightyearAvianPlugin, prelude::*};
use serde::{Deserialize, Serialize};

pub const TIMESTEP: f64 = 1.0 / 64.0; // 64 is a nice number for computers

pub const SERVER_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 5000);

/// The `ProtocolPlugin` is shared between server and client and helps keeps things in sync
/// especially since this is a server-authoritative setup
pub struct ProtocolPlugin;

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        app.component::<PlayerMarker>().replicate();

        app.insert_resource(Gravity::ZERO); // no gravity for now because we have no ground

        // look at line 88 of protocol.rs of the avian_3d example
        app.component::<Position>()
            .replicate()
            .predict()
            .with_rollback_condition(position_should_rollback)
            .add_linear_interpolation()
            .add_correction();
        app.component::<LinearVelocity>()
            .replicate()
            .predict()
            .with_rollback_condition(linear_velocity_should_rollback);

        app.add_plugins(input::native::InputPlugin::<Inputs>::default());
        app.add_plugins((
            PhysicsPlugins::default() // avian physics
                .build()
                .disable::<IslandPlugin>() // sleeping physics objects that ruin determinism
                .disable::<IslandSleepingPlugin>() // no sleeping, please determinism
                .disable::<PhysicsInterpolationPlugin>(), // avian's smoothing that can conflict with lightyear
            LightyearAvianPlugin::default(), // this MUST be added yourself!!!
        ));
    }
}

fn linear_velocity_should_rollback(this: &LinearVelocity, that: &LinearVelocity) -> bool {
    (this.0 - that.0).length() >= 0.01
}

fn position_should_rollback(this: &Position, that: &Position) -> bool {
    (this.0 - that.0).length() >= 0.01
}

// lots of components required for replicating things
#[derive(Component, Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct PlayerMarker;

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
