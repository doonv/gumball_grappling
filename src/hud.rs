use bevy::prelude::*;

use crate::GameState;

pub struct HudPlugin;
impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_hud);
    }
}

fn setup_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        ..default()
    }).with_children(|commands| {
        // TODO make hud or something
    });
    commands.spawn(ImageBundle {
        image: UiImage::new(asset_server.load("textures/crosshair.png")),
        style: Style {
            position_type: PositionType::Absolute,
            margin: UiRect::all(Val::Auto),
            ..default()
        },
        ..default()
    });
}