use bevy::prelude::*;
pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        println!("Client Plugin added!"); // TODO: Remove debug logging
    }
}
