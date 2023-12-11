#![allow(clippy::type_complexity)]

use std::default;

use crate::menu::MenuPlugin;
use crate::player::PlayerPlugin;

mod hud;
mod materials;
mod menu;
mod player;
mod shop;
mod spawning;

use bevy::app::App;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_atmosphere::collection::gradient::Gradient;
use bevy_atmosphere::model::AtmosphereModel;
use bevy_atmosphere::plugin::AtmospherePlugin;
use bevy_toon_shader::ToonShaderPlugin;
use bevy_xpbd_3d::plugins::{PhysicsDebugPlugin, PhysicsPlugins};
use bevy_xpbd_3d::resources::Gravity;
use hud::HudPlugin;
use materials::CustomMaterialsPlugin;
use shop::ShopPlugin;
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
            .insert_resource(Msaa::Off)
            .insert_resource(ClearColor(Color::rgb(0.6, 0.8, 0.9)))
            .insert_resource(AmbientLight {
                brightness: 0.9,
                ..default()
            })
            .insert_resource(Gravity(Vec3::Y * -1.0))
            .insert_resource(AtmosphereModel::new(Gradient {
                sky: Color::rgb(0.6, 0.8, 0.9),
                horizon: Color::rgb(0.8, 0.7, 0.9),
                ground: Color::GRAY,
            }))
            .add_plugins((
                MenuPlugin,
                PlayerPlugin,
                PhysicsPlugins::default(),
                SpawnPlugin,
                // PhysicsDebugPlugin::default(),
                HudPlugin,
                ToonShaderPlugin,
                AtmospherePlugin,
                FrameTimeDiagnosticsPlugin,
                CustomMaterialsPlugin,
                ShopPlugin,
            ));
    }
}
