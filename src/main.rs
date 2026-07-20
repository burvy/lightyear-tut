use bevy::{
    prelude::*,
    render::{
        settings::{Backends, RenderCreation, WgpuSettings},
        RenderPlugin,
    },
};
use lightyear::prelude::*;
use std::io;

mod client;
mod protocol;
mod server;
mod shared;

#[derive(Debug)]
pub enum Mode {
    Client,
    Server,
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(RenderPlugin {
        render_creation: RenderCreation::Automatic(Box::new(WgpuSettings {
            backends: Some(Backends::VULKAN),
            ..default()
        })),
        ..default()
    }));
    let mode;
    println!("Choose:\n1. Client\n2. Server\n");
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Couldn't read");
    let choice: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid Choice, not a number");
            return;
        }
    };
    match choice {
        1 => {
            mode = Mode::Client;
            app.add_plugins(client::ClientPlugin);
        }
        2 => {
            mode = Mode::Server;
            app.add_plugins(server::ServerPlugin);
        }
        _ => {
            panic!("Invalid Choice! Only choices 1 & 2 are available!")
        }
    }
    println!("You have chosen mode {:?}", mode);
    app.add_plugins(protocol::ProtocolPlugin);
    app.run();
}
