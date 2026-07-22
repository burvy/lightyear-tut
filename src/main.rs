use bevy::{
    prelude::*,
    render::{
        settings::{Backends, RenderCreation, WgpuSettings},
        RenderPlugin,
    },
};

use std::io;

mod client;
mod protocol;
mod server;

#[derive(Debug)]
pub enum Mode {
    Client,
    Server,
}

fn main() {
    let mut app = App::new();
    // switching backend to Vulkan to get it to work on my machine
    app.add_plugins(DefaultPlugins.set(RenderPlugin {
        render_creation: RenderCreation::Automatic(Box::new(WgpuSettings {
            backends: Some(Backends::VULKAN),
            ..default()
        })),
        ..default()
    }));

    // mode will store whether we are a client or a server right now
    let mode;

    // let user choose client or server
    println!("Choose:\n1. Server\n2. Client\n");
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
            mode = Mode::Server;
            // run client app if client
            app.add_plugins(server::ServerPlugin);
        }
        2 => {
            mode = Mode::Client;
            // run server app if server
            app.add_plugins(client::ClientPlugin);
        }
        _ => {
            panic!("Invalid Choice! Only choices 1 & 2 are available!")
        }
    }
    println!("You have chosen mode {:?}", mode);

    // protocolplugin must be added last
    app.add_plugins(protocol::ProtocolPlugin);
    app.run();
}
