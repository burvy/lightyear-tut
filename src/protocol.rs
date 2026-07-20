use avian3d::prelude::*;
use bevy::prelude::*;

pub struct ProtocolPlugin;

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        println!("Protocol Plugin added!"); // TODO: Remove debug logging
    }
}
pub enum CharacterWant {
    Move,
    Jump,
    Fire,
}
