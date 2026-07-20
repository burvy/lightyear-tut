use std::time::Duration;

use bevy::prelude::*;
use lightyear::prelude::client::ClientPlugins;

use crate::protocol;
pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        println!("Client Plugin added!"); // TODO: Remove debug logging
        app.add_plugins(ClientPlugins {
            tick_duration: Duration::from_secs_f64(protocol::TIMESTEP),
        });
    }
}
