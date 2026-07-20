use std::time::Duration;

use bevy::prelude::*;
use lightyear::connection::server::Start;
use lightyear::netcode::{server_plugin::NetcodeConfig, NetcodeServer};
use lightyear::prelude::server::{ServerPlugins, ServerUdpIo};
use lightyear::prelude::*;

use crate::protocol::{self, PlayerMarker, SERVER_ADDR};

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        println!("Server Plugin added!"); // TODO: Remove debug logging
        app.add_plugins(ServerPlugins {
            tick_duration: Duration::from_secs_f64(protocol::TIMESTEP),
        });
        app.add_systems(Startup, startup);
        app.add_observer(on_connect);
    }
}

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
fn on_connect(trigger: On<Add, Connected>, mut cmds: Commands) {
    cmds.entity(trigger.entity).insert(ReplicationSender);
    cmds.spawn((
        PlayerMarker,
        Replicate::to_clients(NetworkTarget::All), // sends entity to all players
    ));
}
