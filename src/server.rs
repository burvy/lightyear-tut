use std::time::Duration;

use bevy::prelude::*;
use lightyear::connection::server::Start;
use lightyear::prelude::*;
use lightyear::{
    netcode::{server_plugin::NetcodeConfig, NetcodeServer},
    prelude::server::ServerPlugins,
    webtransport::server::WebTransportServerIo,
};

use crate::protocol::{self, SERVER_ADDR};

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        println!("Server Plugin added!"); // TODO: Remove debug logging
        app.add_plugins(ServerPlugins {
            tick_duration: Duration::from_secs_f64(protocol::TIMESTEP),
        });
        app.add_systems(Startup, startup);
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
