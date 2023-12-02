#![allow(clippy::type_complexity)]


use crate::menu::MenuPlugin;
use crate::player::PlayerPlugin;

mod menu;
mod player;
mod spawning;
mod hud;

use bevy::app::App;
use bevy::prelude::*;
use bevy_xpbd_3d::plugins::{PhysicsPlugins, PhysicsDebugPlugin};
use bevy_xpbd_3d::resources::Gravity;
use hud::HudPlugin;
use spawning::SpawnPlugin;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    #[default]
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .insert_resource(Gravity(Vec3::Y * -5.0))
            .add_plugins((
                MenuPlugin,
                PlayerPlugin,
                PhysicsPlugins::default(),
                SpawnPlugin,
                PhysicsDebugPlugin::default(),
                HudPlugin
            ));
    }
}
