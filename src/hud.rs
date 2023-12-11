use instant::Duration;

use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use crate::{player::Player, shop::PointsSpent, GameState};

#[derive(Resource, Default)]
pub struct Score {
    timer: f32,
    pub current: ScoreData,
    pub to_be_added: ScoreData,
}
#[derive(Default)]
pub struct ScoreData {
    pub height: u64,
    pub destruction: u64,
}
impl ScoreData {
    pub fn total(&self) -> u64 {
        self.height + self.destruction
    }
}
fn transfer_score(mut score: ResMut<Score>, time: Res<Time>) {
    score.timer -= time.delta_seconds();
    if score.timer < 0.0 {
        if score.to_be_added.destruction > 0 {
            score.to_be_added.destruction -= 1;
            score.current.destruction += 1;
        }
        if score.to_be_added.height > 0 {
            score.to_be_added.height -= 1;
            score.current.height += 1;
        }
        score.timer = 0.2;
    }
}

#[derive(Resource, Default)]
pub struct UiHints(Vec<UiHint>);

pub struct UiHint {
    text: Option<String>,
    icon: &'static str,
    duration: Duration,
}

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct InfoText;

#[derive(Component)]
pub struct HintText;
#[derive(Component)]
pub struct HintImage;
#[derive(Component)]
pub struct HintContainer;
#[derive(Component)]
pub struct PauseMenu;
#[derive(Component)]
pub struct PointsSpentText;

pub struct HudPlugin;
impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .init_resource::<UiHints>()
            .add_systems(OnEnter(GameState::Playing), setup_hud)
            .add_systems(
                Update,
                (
                    update_score_text,
                    update_info_text,
                    update_hints,
                    create_hints,
                    transfer_score,
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

fn setup_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Score
    commands.spawn((
        TextBundle {
            text: Text::from_section(
                "0",
                TextStyle {
                    font: asset_server.load("fonts/poppins/Poppins-Thin.ttf"),
                    font_size: 75.0,
                    color: Color::WHITE,
                },
            ),
            style: Style {
                position_type: PositionType::Absolute,
                margin: UiRect {
                    top: Val::ZERO,
                    ..UiRect::all(Val::Auto)
                },
                ..default()
            },
            ..default()
        },
        ScoreText,
    ));
    commands.spawn((
        TextBundle {
            text: Text::from_section(
                "0",
                TextStyle {
                    font: asset_server.load("fonts/poppins/Poppins-Thin.ttf"),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            ),
            style: Style {
                position_type: PositionType::Absolute,
                margin: UiRect {
                    top: Val::Px(60.0),
                    ..UiRect::all(Val::Auto)
                },
                ..default()
            },
            ..default()
        },
        PointsSpentText,
    ));
    // Crosshair
    commands.spawn(ImageBundle {
        image: UiImage::new(asset_server.load("textures/crosshair.png")),
        style: Style {
            position_type: PositionType::Absolute,
            margin: UiRect::all(Val::Auto),
            ..default()
        },
        ..default()
    });
    // Debug info
    commands.spawn((
        TextBundle {
            text: Text::from_sections([
                TextSection {
                    value: "FPS: ".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/poppins/Poppins-Regular.ttf"),
                        font_size: 30.0,
                        color: Color::WHITE,
                    },
                },
                TextSection {
                    value: "N/A".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/poppins/Poppins-Medium.ttf"),
                        font_size: 30.0,
                        color: Color::WHITE,
                    },
                },
                TextSection {
                    value: "\nEntities: ".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/poppins/Poppins-Regular.ttf"),
                        font_size: 30.0,
                        color: Color::WHITE,
                    },
                },
                TextSection {
                    value: "0".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/poppins/Poppins-Medium.ttf"),
                        font_size: 30.0,
                        color: Color::WHITE,
                    },
                },
                TextSection {
                    value: "\nPlayer Y: ".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/poppins/Poppins-Regular.ttf"),
                        font_size: 30.0,
                        color: Color::WHITE,
                    },
                },
                TextSection {
                    value: "0".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/poppins/Poppins-Medium.ttf"),
                        font_size: 30.0,
                        color: Color::WHITE,
                    },
                },
            ]),
            style: Style {
                position_type: PositionType::Absolute,
                margin: UiRect {
                    top: Val::Px(2.0),
                    left: Val::Px(10.0),
                    ..UiRect::all(Val::Auto)
                },
                ..default()
            },
            ..default()
        },
        InfoText,
    ));
    // Hints
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    margin: UiRect {
                        bottom: Val::Vh(25.0),
                        ..UiRect::all(Val::Auto)
                    },
                    ..default()
                },
                visibility: Visibility::Hidden,
                ..default()
            },
            HintContainer,
        ))
        .with_children(|commands| {
            commands.spawn((
                ImageBundle {
                    style: Style {
                        height: Val::Px(40.0),
                        margin: UiRect::right(Val::Px(5.0)),
                        ..default()
                    },
                    ..default()
                },
                HintImage,
            ));
            commands.spawn((
                TextBundle {
                    text: Text::from_section(
                        "",
                        TextStyle {
                            font: asset_server.load("fonts/poppins/Poppins-Thin.ttf"),
                            font_size: 40.0,
                            color: Color::WHITE,
                        },
                    ),

                    ..default()
                },
                HintText,
            ));
        });
    // Pause
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Vw(100.0),
                    height: Val::Vw(100.0),
                    // align_items: AlignItems::Center,
                    // align_content: AlignContent::Center,
                    // justify_content: JustifyContent::Center,
                    // flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: BackgroundColor(Color::rgba(0.0, 0.0, 0.0, 0.75)),
                visibility: Visibility::Hidden,
                ..default()
            },
            PauseMenu,
        ))
        .with_children(|commands| {
            commands.spawn(TextBundle {
                text: Text::from_section(
                    "PAUSE",
                    TextStyle {
                        font: asset_server.load("fonts/poppins/Poppins-Thin.ttf"),
                        font_size: 75.0,
                        color: Color::WHITE,
                    },
                ),
                style: Style {
                    margin: UiRect {
                        bottom: Val::ZERO,
                        ..UiRect::all(Val::Auto)
                    },
                    ..default()
                },
                background_color: BackgroundColor(Color::RED),
                ..default()
            });
            commands.spawn(TextBundle {
                text: Text::from_section(
                    "Continue",
                    TextStyle {
                        font: asset_server.load("fonts/poppins/Poppins-Thin.ttf"),
                        font_size: 50.0,
                        color: Color::WHITE,
                    },
                ),
                style: Style {
                    margin: UiRect {
                        top: Val::ZERO,
                        ..UiRect::all(Val::Auto)
                    },
                    ..default()
                },
                background_color: BackgroundColor(Color::RED),
                ..default()
            });
        });
}

fn update_score_text(
    score: Res<Score>,
    points_spent: Res<PointsSpent>,
    mut texts: Query<&mut Text, (With<ScoreText>, Without<PointsSpentText>)>,
    mut texts2: Query<(&mut Text, &mut Visibility), (Without<ScoreText>, With<PointsSpentText>)>,
) {
    for mut text in texts.iter_mut() {
        text.sections[0].value = score.current.total().to_string();
    }
    for mut text in texts2.iter_mut() {
        if points_spent.0 > 0 {
            text.0.sections[0].value = (score.current.total() - points_spent.0).to_string();
            *text.1 = Visibility::Visible;
        } else {
            *text.1 = Visibility::Hidden;
        }
    }
}
fn update_info_text(
    mut texts: Query<(&mut Text, &mut Visibility), With<InfoText>>,
    player: Query<&Transform, With<Player>>,
    diagnostics: Res<DiagnosticsStore>,
    entities: Query<Entity>,
    key: Res<Input<KeyCode>>,
) {
    let fps = diagnostics
        .get(FrameTimeDiagnosticsPlugin::FPS)
        .map(|v| v.smoothed())
        .flatten();
    let player_transform = player.single();
    for (mut text, mut visibility) in texts.iter_mut() {
        if key.just_pressed(KeyCode::F3) {
            if *visibility == Visibility::Hidden {
                *visibility = Visibility::Inherited;
            } else {
                *visibility = Visibility::Hidden;
            }
        }
        if *visibility != Visibility::Hidden {
            text.sections[1].value = fps.map(|v| format!("{v:.0}")).unwrap_or("N/A".to_string());
            text.sections[3].value = entities.iter().len().to_string();
            text.sections[5].value = format!("{:.1}", player_transform.translation.y);
        }
    }
}
fn update_hints(
    mut visiblity: Query<&mut Visibility, With<HintContainer>>,
    mut text: Query<&mut Text, With<HintText>>,
    mut image: Query<&mut UiImage, With<HintImage>>,
    mut hints: ResMut<UiHints>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    if let Some(hint) = hints.0.first_mut() {
        if let Some(new_durtation) = hint.duration.checked_sub(time.delta()) {
            hint.duration = new_durtation;
        } else {
            hints.0.remove(0);
            if let Ok(mut visiblity) = visiblity.get_single_mut() {
                *visiblity = Visibility::Hidden;
            }
            return;
        }
        if let Some(hint_text) = hint.text.take() {
            if let Ok(mut visiblity) = visiblity.get_single_mut() {
                *visiblity = Visibility::Visible;
            }
            if let Ok(mut text) = text.get_single_mut() {
                text.sections[0].value = hint_text;
            }
            if let Ok(mut image) = image.get_single_mut() {
                image.texture = asset_server.load(hint.icon);
            }
        }
    }
}

fn create_hints(
    mut hints: ResMut<UiHints>,
    player: Query<&Transform, With<Player>>,
    mut hints_activated: Local<[bool; 3]>,
) {
    if let Ok(player) = player.get_single() {
        // trace!("{:.1}", player.translation.y);
        if player.translation.y > 0.6 && !hints_activated[0] {
            hints.0.push(UiHint {
                text: Some("to use your grappling hook on an object.".to_string()),
                icon: "textures/keyboardmouse/Mouse_Left_Key_Dark.png",
                duration: Duration::from_secs(5),
            });
            hints_activated[0] = true;
        }
        if player.translation.y > 100.0 && !hints_activated[1] {
            hints.0.push(UiHint {
                text: Some("to dash. (Replinishes upon hooking to an object)".to_string()),
                icon: "textures/keyboardmouse/Mouse_Right_Key_Dark.png",
                duration: Duration::from_secs(5),
            });
            hints_activated[1] = true;
        }
        if player.translation.y > 300.0 && !hints_activated[2] {
            hints.0.push(UiHint {
                text: Some("to toggle additional info.".to_string()),
                icon: "textures/keyboardmouse/F3_Key_Dark.png",
                duration: Duration::from_secs(5),
            });
            hints_activated[2] = true;
        }
    }
}
