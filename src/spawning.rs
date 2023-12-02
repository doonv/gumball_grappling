use bevy::{pbr::DirectionalLightShadowMap, prelude::*};
use bevy_xpbd_3d::prelude::*;
use rand::Rng;

use crate::GameState;

pub const DESPAWN_Y: f32 = -100.0;
pub const SPAWN_EVERY_SECONDS: f64 = 0.01;

#[derive(Component)]
pub struct DespawnOnLowerThanY;

pub struct SpawnPlugin;
impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DirectionalLightShadowMap { size: 4096 })
            .add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(
                Update,
                (spawn_falling_objects, despawn_falling_objects)
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 50000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 1.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(10.0, 1.0, 10.0))),
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            transform: Transform::from_xyz(0.0, -1.0, 0.0),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(10.0, 1.0, 10.0),
    ));
}

pub fn spawn_falling_objects(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
    mut mesh_container: Local<Option<Handle<Mesh>>>,
    mut last_time_elapsed: Local<f64>,
) {
    let mut rand = rand::thread_rng();
    if mesh_container.is_none() {
        *mesh_container = Some(meshes.add(Mesh::from(shape::Cube::new(1.0))));
    }
    let mesh = mesh_container.as_ref().unwrap();

    while *last_time_elapsed < time.elapsed_seconds_f64() {
        commands.spawn((
            PbrBundle {
                mesh: mesh.clone(),
                material: materials.add(
                    Color::rgb(
                        rand.gen_range(0.0..1.0),
                        rand.gen_range(0.0..1.0),
                        rand.gen_range(0.0..1.0),
                    )
                    .into(),
                ),
                transform: Transform::from_xyz(
                    rand.gen_range(-200.0..200.0),
                    100.0,
                    rand.gen_range(-200.0..200.0),
                ),
                ..default()
            },
            RigidBody::Dynamic,
            Collider::cuboid(1.0, 1.0, 1.0),
            DespawnOnLowerThanY,
        ));
        *last_time_elapsed += SPAWN_EVERY_SECONDS;
    }
}

fn despawn_falling_objects(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<DespawnOnLowerThanY>>,
) {
    for (entity, transform) in query.iter() {
        if transform.translation.y < DESPAWN_Y {
            commands.entity(entity).despawn();
        }
    }
}
