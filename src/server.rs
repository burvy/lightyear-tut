use std::time::Duration;

use bevy::prelude::*;
use lightyear::prelude::server::ServerPlugins;

use crate::protocol;

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        println!("Server Plugin added!"); // TODO: Remove debug logging
        app.add_plugins(ServerPlugins {
            tick_duration: Duration::from_secs_f64(protocol::TIMESTEP),
        });
    }
}
