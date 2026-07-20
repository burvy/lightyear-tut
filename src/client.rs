use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use bevy::prelude::*;
use lightyear::netcode::Key;
use lightyear::prelude::client::ClientPlugins;
use lightyear::prelude::*;
use lightyear::{
    connection::client::Connect,
    link::Link,
    netcode::{auth::Authentication, client_plugin::NetcodeConfig, NetcodeClient},
};

use crate::protocol::{self, PlayerMarker, SERVER_ADDR};

/// Client address, each client must have a unique port. 0 lets the OS choose any available one
const CLIENT_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0);
pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        println!("Client Plugin added!"); // TODO: Remove debug logging
        app.add_plugins(ClientPlugins {
            tick_duration: Duration::from_secs_f64(protocol::TIMESTEP),
        });
        app.add_systems(Startup, startup);
        app.add_observer(|_: On<Add, PlayerMarker>| info!("a player was replicated to me!"));
    }
}

fn startup(mut cmds: Commands) -> Result {
    let auth = Authentication::Manual {
        server_addr: SERVER_ADDR,
        client_id: SystemTime::now() // sets id as current nanosecond time (rarely overlaps)
            .duration_since(UNIX_EPOCH)
            .expect("Couldn't set time")
            .as_nanos() as u64,
        private_key: Key::default(),
        protocol_id: 0,
    };
    let client = cmds
        .spawn((
            Client::default(),
            LocalAddr(CLIENT_ADDR),
            PeerAddr(SERVER_ADDR),
            Link::default(),
            ReplicationReceiver,
            NetcodeClient::new(auth, NetcodeConfig::default())?,
            UdpIo::default(),
        ))
        .id();
    cmds.trigger(Connect { entity: client });
    Ok(())
}
