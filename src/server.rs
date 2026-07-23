use std::time::Duration;

use bevy::prelude::*;
use lightyear::connection::client_of::ClientOf;
use lightyear::connection::server::Start;
use lightyear::netcode::{server_plugin::NetcodeConfig, NetcodeServer};
use lightyear::prelude::input::native::ActionState;
use lightyear::prelude::server::{ServerPlugins, ServerUdpIo};
use lightyear::prelude::*;

use crate::protocol::{self, Inputs, PlayerMarker, PlayerPosition, SERVER_ADDR};
use crate::shared;

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        println!("Server Plugin added!"); // TODO: Remove debug logging
        app.add_plugins(ServerPlugins {
            tick_duration: Duration::from_secs_f64(protocol::TIMESTEP),
        });
        app.add_systems(Startup, startup);
        app.add_systems(FixedUpdate, movement);
        app.add_observer(on_connect);
    }
}

/// Spawn the server
fn startup(mut cmds: Commands) -> Result {
    let server = cmds
        .spawn((
            NetcodeServer::new(NetcodeConfig::default()),
            LocalAddr(SERVER_ADDR),
            ServerUdpIo::default(),
        ))
        .id();

    cmds.trigger(Start { entity: server });
    Ok(())
}

/// runs when something that has Connected has been added
fn on_connect(
    trigger: On<Add, Connected>,
    query: Query<&RemoteId, With<ClientOf>>,
    mut cmds: Commands,
) {
    // copied from simple_box's server.rs line 55
    let Ok(client_id) = query.get(trigger.entity) else {
        return;
    };
    let client_id = client_id.0; // a PeerId
    cmds.entity(trigger.entity).insert(ReplicationSender);
    cmds.spawn((
        PlayerMarker,
        PlayerPosition(Vec3::ZERO),
        PredictionTarget::to_clients(NetworkTarget::Single(client_id)),
        ControlledBy {
            owner: trigger.entity, // lightyear automatically links player to server entity
            lifetime: Default::default(), // when player disconnects
        },
        Replicate::to_clients(NetworkTarget::All), // sends entity to all players
    ));
}

/// Flow Point 2
/// the client's inputs are directly written onto ActionState
fn movement(mut query: Query<(&mut PlayerPosition, &ActionState<Inputs>)>) {
    query.iter_mut().for_each(|(mut pos, action)| {
        shared::apply_input(&mut pos, &action.0); // shared between both server and client
    });
}
