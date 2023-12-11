use bevy::{
    ecs::system::{Command, RunSystemOnce},
    prelude::*,
    ui::widget::UiImageSize,
};
use bevy_xpbd_3d::plugins::setup::{Physics, PhysicsTime};

use crate::{hud::Score, materials::RoundedRectangleMaterial, player::Player, GameState};

#[derive(Resource, Default)]
pub struct PointsSpent(pub u64);

#[derive(Resource, Default)]
pub struct IsShopping(bool);

#[derive(Component)]
pub struct Shop;

#[derive(Component)]
pub struct HookRangeText;

#[derive(Component)]
pub struct HookStrengthText;

#[derive(Component)]
pub struct DashStrengthText;

pub struct ShopPlugin;
impl Plugin for ShopPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<IsShopping>()
            .init_resource::<PointsSpent>()
            .add_systems(
                Update,
                ((enter_exit_shop, do_upgrades).run_if(in_state(GameState::Playing)),),
            );
    }
}
fn enter_exit_shop(
    mut shopping: ResMut<IsShopping>,
    key: Res<Input<KeyCode>>,
    mut commands: Commands,
) {
    if key.just_pressed(KeyCode::Tab) {
        if shopping.0 {
            shopping.0 = false;
            commands.add(DespawnShop);
        } else {
            shopping.0 = true;
            commands.add(SetupShop);
        }
    }
}

pub struct SetupShop;
impl Command for SetupShop {
    fn apply(self, world: &mut World) {
        fn setup_shop(
            mut commands: Commands,
            asset_server: Res<AssetServer>,
            mut rectangles: ResMut<Assets<RoundedRectangleMaterial>>,
        ) {
            commands
                .spawn((
                    MaterialNodeBundle {
                        material: rectangles.add(RoundedRectangleMaterial {
                            color: Color::rgb(
                                0.17 * 0.3826086956521739,
                                0.2 * 0.3826086956521739,
                                0.25 * 0.3826086956521739,
                            )
                            .into(),
                            roundedness: Vec2::new(2.0, 0.2),
                        }),
                        style: Style {
                            position_type: PositionType::Absolute,
                            height: Val::Px(30.),
                            left: Val::Px(10.0),
                            right: Val::Px(10.0),
                            bottom: Val::Px(10.0),
                            padding: UiRect::horizontal(Val::Px(5.0)),
                            flex_direction: FlexDirection::Row,

                            ..default()
                        },
                        ..default()
                    },
                    Shop,
                ))
                .with_children(|commands| {
                    let style = TextStyle {
                        font: asset_server.load("fonts/poppins/Poppins-Thin.ttf"),
                        font_size: 30.0,
                        color: Color::WHITE,
                    };
                    commands.spawn(ImageBundle {
                        image: UiImage::new(
                            asset_server.load("textures/keyboardmouse/1_Key_Dark.png"),
                        ),
                        ..default()
                    });
                    commands.spawn((
                        TextBundle::from_sections([
                            TextSection::new("Hook Range ", style.clone()),
                            TextSection::new("?", style.clone()),
                            TextSection::new(" - ", style.clone()),
                            TextSection::new("?", style.clone()),
                            TextSection::new(" points", style.clone()),
                        ]),
                        HookRangeText,
                    ));
                    commands.spawn(ImageBundle {
                        image: UiImage::new(
                            asset_server.load("textures/keyboardmouse/2_Key_Dark.png"),
                        ),
                        style: Style {
                            margin: UiRect::left(Val::Px(20.0)),
                            ..default()
                        },
                        ..default()
                    });
                    commands.spawn((
                        TextBundle::from_sections([
                            TextSection::new("Hook Strength ", style.clone()),
                            TextSection::new("?", style.clone()),
                            TextSection::new(" - ", style.clone()),
                            TextSection::new("?", style.clone()),
                            TextSection::new(" points", style.clone()),
                        ]),
                        HookStrengthText,
                    ));
                    commands.spawn(ImageBundle {
                        image: UiImage::new(
                            asset_server.load("textures/keyboardmouse/3_Key_Dark.png"),
                        ),
                        style: Style {
                            margin: UiRect::left(Val::Px(20.0)),
                            ..default()
                        },
                        ..default()
                    });
                    commands.spawn((
                        TextBundle::from_sections([
                            TextSection::new("Dash Strength ", style.clone()),
                            TextSection::new("?", style.clone()),
                            TextSection::new(" - ", style.clone()),
                            TextSection::new("?", style.clone()),
                            TextSection::new(" points", style.clone()),
                        ]),
                        DashStrengthText,
                    ));
                });
        }
        world.run_system_once(setup_shop);
    }
}

pub struct DespawnShop;
impl Command for DespawnShop {
    fn apply(self, world: &mut World) {
        fn despawn_shop(shops: Query<Entity, With<Shop>>, mut commands: Commands) {
            for shop in shops.iter() {
                commands.entity(shop).despawn_recursive();
            }
        }
        world.run_system_once(despawn_shop);
    }
}

fn do_upgrades(
    key: Res<Input<KeyCode>>,
    is_shopping: Res<IsShopping>,
    points: Res<Score>,
    mut points_spent: ResMut<PointsSpent>,
    mut player: Query<&mut Player>,
    mut hook_range_text: Query<
        &mut Text,
        (
            With<HookRangeText>,
            Without<HookStrengthText>,
            Without<DashStrengthText>,
        ),
    >,
    mut hook_strength_text: Query<
        &mut Text,
        (
            Without<HookRangeText>,
            With<HookStrengthText>,
            Without<DashStrengthText>,
        ),
    >,
    mut dash_strength_text: Query<
        &mut Text,
        (
            Without<HookRangeText>,
            Without<HookStrengthText>,
            With<DashStrengthText>,
        ),
    >,
) {
    if !is_shopping.0 {
        return;
    }
    let player_upgrades = &mut player.single_mut().upgrades;
    let points_to_be_spent = points.current.total() - points_spent.0;
    let get_price = |x| (x * x * 3) + 10;

    if let Ok(mut hook_range_text) = hook_range_text.get_single_mut() {
        hook_range_text.sections[1].value = (player_upgrades.hook_range + 1).to_string();
        hook_range_text.sections[3].value = get_price(player_upgrades.hook_range).to_string();
    }
    if let Ok(mut hook_strength_text) = hook_strength_text.get_single_mut() {
        hook_strength_text.sections[1].value = (player_upgrades.hook_strength + 1).to_string();
        hook_strength_text.sections[3].value = get_price(player_upgrades.hook_strength).to_string();
    }
    if let Ok(mut dash_strength_text) = dash_strength_text.get_single_mut() {
        dash_strength_text.sections[1].value = (player_upgrades.dash_strength + 1).to_string();
        dash_strength_text.sections[3].value = get_price(player_upgrades.dash_strength).to_string();
    }

    if key.just_pressed(KeyCode::Key1) {
        if points_to_be_spent > get_price(player_upgrades.hook_range) {
            points_spent.0 += get_price(player_upgrades.hook_range);
            player_upgrades.hook_range += 1;
        }
    } else if key.just_pressed(KeyCode::Key2) {
        if points_to_be_spent > get_price(player_upgrades.hook_strength) {
            points_spent.0 += get_price(player_upgrades.hook_strength);
            player_upgrades.hook_strength += 1;
        }
    } else if key.just_pressed(KeyCode::Key3) {
        if points_to_be_spent > get_price(player_upgrades.dash_strength) {
            points_spent.0 += get_price(player_upgrades.dash_strength);
            player_upgrades.dash_strength += 1;
        }
    }
}
