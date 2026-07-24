use std::time::Duration;

use bevy::prelude::*;
use lightyear::connection::client_of::ClientOf;
use lightyear::connection::server::Start;
use lightyear::netcode::{server_plugin::NetcodeConfig, NetcodeServer};
use lightyear::prelude::input::native::ActionState;
use lightyear::prelude::server::ServerPlugins;
use lightyear::prelude::*;
use lightyear::webtransport::server::WebTransportServerIo;

use crate::protocol::{self, Inputs, PlayerMarker, PlayerPosition, SERVER_ADDR};
use crate::shared;

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
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
    let valid_addresses = vec![
        "localhost".to_string(),
        "127.0.0.1".to_string(),
        "::1".to_string(),
    ];
    let identity = Identity::self_signed(valid_addresses)?;
    let digest = identity.certificate_chain().as_slice()[0].hash(); // the fingerprint
    let digest_hex = digest.to_string().replace(":", "");
    std::fs::write("digest.txt", &digest_hex)?;
    info!("cert digest {digest}");
    let server = cmds
        .spawn((
            NetcodeServer::new(NetcodeConfig::default()),
            LocalAddr(SERVER_ADDR),
            WebTransportServerIo {
                certificate: identity,
            },
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
        // replicate player entity to all clients (copied from server.rs line 62 simple_box)
        Replicate::to_clients(NetworkTarget::All),
        PredictionTarget::to_clients(NetworkTarget::Single(client_id)),
        InterpolationTarget::to_clients(NetworkTarget::AllExceptSingle(client_id)),
        ControlledBy {
            owner: trigger.entity,
            lifetime: Default::default(),
        },
    ));
}

/// Flow Point 2
/// the client's inputs are directly written onto ActionState
fn movement(mut query: Query<(&mut PlayerPosition, &ActionState<Inputs>)>) {
    query.iter_mut().for_each(|(mut pos, action)| {
        shared::apply_input(&mut pos, &action.0); // shared between both server and client
    });
}
