use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use bevy::prelude::*;
use lightyear::input::client::InputSystems;
use lightyear::netcode::Key;
use lightyear::prelude::client::ClientPlugins;
use lightyear::prelude::input::native::{ActionState, InputMarker};
use lightyear::prelude::*;
use lightyear::{
    connection::client::Connect,
    link::Link,
    netcode::{auth::Authentication, client_plugin::NetcodeConfig, NetcodeClient},
};

use crate::protocol::{self, Inputs, PlayerMarker, PlayerPosition, SERVER_ADDR};
use crate::shared;

/// Client address, each client must have a unique port. 0 lets the OS choose any available one
const CLIENT_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0);

/// The client side app
pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ClientPlugins {
            tick_duration: Duration::from_secs_f64(protocol::TIMESTEP), // serv and client must share timestep from protocol
        });
        app.add_systems(
            Startup, // initialize world
            (startup, player_scene.spawn(), world_scene.spawn()),
        );

        app.add_systems(
            FixedPreUpdate,
            buffer_input.in_set(InputSystems::WriteClientInputs), // actually sends info to the server
        );
        app.add_systems(FixedUpdate, player_movement); // updates ourselves client side to avoid lag
        app.add_systems(
            Update,
            (
                draw_players, // draw players when they initially get detected
                sync_players, // move players according to the confirmed positions
            ),
        );
        app.add_observer(|_: On<Add, PlayerMarker>| info!("a player was replicated to me!"));

        // Flow Point 1
        // `WriteClientInputs` prepares inputs (up, down ..) on entity
        // i believe then lightyear handles and
        // sends the messages automatically through some pipeline
        app.add_observer(|t: On<Add, Controlled>, mut cmds: Commands| {
            cmds.entity(t.entity)
                .insert(InputMarker::<Inputs>::default());
        });
    }
}

/// Automatically joins the server set with the SERVER_ADDR
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
            PredictionManager::default(), // required for prediction
        ))
        .id();
    cmds.trigger(Connect { entity: client });
    Ok(())
}

/// build the player (spawn things)
fn player_scene() -> impl Scene {
    let p_transform =
        Transform::from_translation(Vec3::new(0.0, 5.0, 5.0)).looking_at(Vec3::ZERO, Vec3::Y);
    bsn! {
        #LocalPlayer
        Camera3d
        // wrap whatever you want in template_value() to force evaluate it
        template_value(p_transform)
    }
}

/// build the client side world, including the light so the player can actually see
fn world_scene() -> impl Scene {
    let l_transform =
        Transform::from_translation(Vec3::new(0.0, 5.0, 10.0)).looking_at(Vec3::ZERO, Vec3::Y);
    bsn! {
        DirectionalLight {
            color: Color::WHITE,
            illuminance: 10000.0,
        }
        template_value(l_transform)

    }
}

/// drawing newly `added` playerpositions
fn draw_players(
    mut cmds: Commands,
    players: Query<
        Entity,
        (
            Added<PlayerPosition>,
            Or<(With<Predicted>, With<Interpolated>)>,
        ),
    >,
) {
    players.iter().for_each(|entity| {
        cmds.entity(entity).queue_apply_scene(bsn! {
            Mesh3d(asset_value(Cuboid::from_length(1.0)))
            MeshMaterial3d::<StandardMaterial>(asset_value(Color::WHITE))
        });
    });
}

/// moving existing player transforms to their respective updated playerpositions
fn sync_players(
    mut query: Query<(&PlayerPosition, &mut Transform), Or<(With<Predicted>, With<Interpolated>)>>,
) {
    query.iter_mut().for_each(|(pos, mut transform)| {
        transform.translation = pos.0;
    })
}

fn buffer_input(
    mut query: Query<&mut ActionState<Inputs>, With<InputMarker<Inputs>>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if let Ok(mut action) = query.single_mut() {
        action.0 = Inputs {
            up: keys.pressed(KeyCode::KeyW),
            down: keys.pressed(KeyCode::KeyS),
            left: keys.pressed(KeyCode::KeyA),
            right: keys.pressed(KeyCode::KeyD),
        };
    }
}

/// predict movement client side (reads OUR actionstate inputs)
fn player_movement(mut query: Query<(&mut PlayerPosition, &ActionState<Inputs>), With<Predicted>>) {
    query.iter_mut().for_each(|(mut pos, action)| {
        shared::apply_input(&mut pos, &action.0);
    })
}
