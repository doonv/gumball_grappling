use bevy::{
    ecs::system::{Command, RunSystemOnce},
    prelude::*,
};
use bevy_xpbd_3d::plugins::setup::{Physics, PhysicsTime};

use crate::{materials::RoundedRectangleMaterial, GameState};

#[derive(Resource)]
pub struct PointsSpent(u64);

#[derive(Resource, Default)]
pub struct IsShopping(bool);

#[derive(Component)]
pub struct Shop;

pub struct ShopPlugin;
impl Plugin for ShopPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<IsShopping>().add_systems(
            Update,
            (enter_exit_shop.run_if(in_state(GameState::Playing)),),
        );
    }
}
fn enter_exit_shop(
    mut shopping: ResMut<IsShopping>,
    key: Res<Input<KeyCode>>,
    mut physics_time: ResMut<Time<Physics>>,
    mut commands: Commands,
) {
    if key.just_pressed(KeyCode::Tab) {
        if shopping.0 {
            physics_time.unpause();
            shopping.0 = false;
            commands.add(DespawnShop);
        } else {
            physics_time.pause();
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
            commands.spawn((
                MaterialNodeBundle {
                    material: rectangles.add(RoundedRectangleMaterial {
                        color: Color::rgb(0.1, 0.1, 0.2).into(),
                        roundedness: Vec2::new(0.2 * 1.5, 0.2),
                    }),
                    style: Style {
                        position_type: PositionType::Absolute,
                        height: Val::Vh(90.0),
                        margin: UiRect::all(Val::Auto),
                        aspect_ratio: Some(1.5 / 1.),
                        ..default()
                    },
                    ..default()
                },
                Shop,
            ));
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
