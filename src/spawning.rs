use bevy::{pbr::DirectionalLightShadowMap, prelude::*};
use bevy_toon_shader::{ToonShaderMaterial, ToonShaderSun};
use bevy_xpbd_3d::prelude::*;
use rand::Rng;

use crate::{materials::OutlineToonMaterial, player::Player, GameState};

pub const DESPAWN_Y: f32 = -100.0;
pub const MIN_SPHERE_DISTANCE: f32 = 1000.0;
pub const MIN_THINGAMAJIG_DISTANCE: f32 = 5000.0;

#[derive(Resource)]
pub struct SpawnSettings {
    lvl1_spawn: f64,
    lvl2_spawn: f64,
}

#[derive(Component)]
pub struct DespawnOnLowerThanY;

pub struct SpawnPlugin;
impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DirectionalLightShadowMap { size: 4096 })
            .insert_resource(SpawnSettings {
                lvl1_spawn: 0.02,
                lvl2_spawn: 0.0,
            })
            .add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(
                Update,
                (
                    spawn_falling_objects,
                    despawn_falling_objects,
                    modify_spawn_settings,
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut toon_materials: ResMut<Assets<ToonShaderMaterial>>,
) {
    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 50000.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(0.4, 1.0, 0.7).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        ToonShaderSun,
    ));
    commands.spawn((
        MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(10.0, 1.0, 10.0))),
            material: toon_materials.add(ToonShaderMaterial {
                color: Color::GRAY,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, -1.0, 0.0),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(10.0, 1.0, 10.0),
    ));
}

#[derive(Component)]
pub struct StaticSphere;

#[derive(Component)]
pub struct Thingajamig;

pub fn spawn_falling_objects(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut toon_materials: ResMut<Assets<OutlineToonMaterial>>,
    time: Res<Time>,
    physics_time: Res<Time<Physics>>,
    player: Query<&Transform, With<Player>>,
    mut last_time_elapsed_lvl1: Local<f64>,
    mut last_time_elapsed_lvl2: Local<f64>,
    spheres: Query<&Transform, With<StaticSphere>>,
    thingamajigs: Query<&Transform, With<Thingajamig>>,
    spawn_settings: Res<SpawnSettings>,
    mut mesh_material_container: Local<Option<(Handle<Mesh>, Handle<Mesh>, Handle<Mesh>)>>,
) {
    if physics_time.is_paused() {
        *last_time_elapsed_lvl1 = time.elapsed_seconds_f64();
        *last_time_elapsed_lvl2 = time.elapsed_seconds_f64();
        return;
    }
    let player_transform = player.single();
    let mut rand = rand::thread_rng();

    if mesh_material_container.is_none() {
        *mesh_material_container = Some((
            meshes.add(shape::Cube::new(1.0).into()),
            meshes.add(
                shape::UVSphere {
                    radius: 1.0,
                    sectors: 24,
                    stacks: 12,
                }
                .into(),
            ),
            meshes.add(
                shape::UVSphere {
                    radius: 3.0,
                    sectors: 72,
                    stacks: 36,
                }
                .into(),
            ),
        ));
    }
    let (cube_mesh, sphere_1_mesh, sphere_3_mesh) = mesh_material_container.as_ref().unwrap();

    while *last_time_elapsed_lvl1 < time.elapsed_seconds_f64() {
        if rand.gen_bool(0.1) {
            let pos = Vec3::new(
                rand.gen_range(-100.0..100.0) + player_transform.translation.x,
                rand.gen_range(-100.0..100.0) + player_transform.translation.y,
                rand.gen_range(-100.0..100.0) + player_transform.translation.z,
            );
            let other_sphere_nearby = spheres
                .iter()
                .any(|v| v.translation.distance_squared(pos) < MIN_SPHERE_DISTANCE);
            if !other_sphere_nearby {
                commands.spawn((
                    MaterialMeshBundle {
                        mesh: sphere_3_mesh.clone(),
                        material: toon_materials.add(OutlineToonMaterial {
                            color: Color::GRAY,
                            outline_color: Color::NONE,
                            ..default()
                        }),
                        transform: Transform::from_translation(pos),
                        ..default()
                    },
                    RigidBody::Static,
                    Collider::ball(3.0),
                    DespawnOnLowerThanY,
                    StaticSphere,
                ));
            }
        } else {
            commands.spawn((
                MaterialMeshBundle {
                    mesh: sphere_1_mesh.clone(),
                    material: toon_materials.add(OutlineToonMaterial {
                        color: Color::rgb_linear(
                            rand.gen_range(0.3..10.0),
                            rand.gen_range(0.3..10.0),
                            rand.gen_range(0.3..10.0),
                        ),
                        outline_color: Color::NONE,
                        ..default()
                    }),
                    transform: Transform::from_xyz(
                        rand.gen_range(-200.0..200.0) + player_transform.translation.x,
                        100.0 + player_transform.translation.y,
                        rand.gen_range(-200.0..200.0) + player_transform.translation.z,
                    ),
                    ..default()
                },
                RigidBody::Dynamic,
                LinearVelocity(Vec3::Y * -10.0),
                Collider::ball(1.0),
                DespawnOnLowerThanY,
            ));
        }

        *last_time_elapsed_lvl1 += spawn_settings.lvl1_spawn;
    }
    if spawn_settings.lvl2_spawn == 0.0 {
        *last_time_elapsed_lvl2 = time.elapsed_seconds_f64();
    }
    while *last_time_elapsed_lvl2 < time.elapsed_seconds_f64() {
        if rand.gen_bool(0.1) {
            let pos = Vec3::new(
                rand.gen_range(-200.0..200.0) + player_transform.translation.x,
                100.0 + player_transform.translation.y,
                rand.gen_range(-200.0..200.0) + player_transform.translation.z,
            );
            let other_thingamajig_nearby = thingamajigs
                .iter()
                .any(|v| v.translation.distance_squared(pos) < MIN_THINGAMAJIG_DISTANCE);
            let material_color = Color::rgb_linear(
                rand.gen_range(0.3..10.0),
                rand.gen_range(0.3..10.0),
                rand.gen_range(0.3..10.0),
            );
            let material = toon_materials.add(OutlineToonMaterial {
                color: material_color,
                outline_color: Color::NONE,
                ..default()
            });
            if !other_thingamajig_nearby {
                info!("spawning!");
                commands.spawn((Thingajamig, Transform::from_translation(pos)));
                for x in 0..10 {
                    for y in 0..10 {
                        for z in 0..5 {
                            commands.spawn((
                                MaterialMeshBundle {
                                    mesh: cube_mesh.clone(),
                                    material: material.clone(),
                                    transform: Transform::from_xyz(
                                        pos.x + (x as f32 * 2.0),
                                        pos.y + (y as f32 * 2.0),
                                        pos.z + (z as f32 * 2.0),
                                    ),
                                    ..default()
                                },
                                RigidBody::Dynamic,
                                GravityScale(0.0),
                                Collider::cuboid(1.0, 1.0, 1.0),
                                DespawnOnLowerThanY,
                            ));
                        }
                    }
                }
            }
        }
        *last_time_elapsed_lvl2 += spawn_settings.lvl2_spawn;
    }
}

fn despawn_falling_objects(
    mut commands: Commands,
    player: Query<&Transform, With<Player>>,
    query: Query<(Entity, &Transform), With<DespawnOnLowerThanY>>,
) {
    let player_transform = player.single();
    for (entity, transform) in query.iter() {
        if transform.translation.y < DESPAWN_Y + player_transform.translation.y {
            commands.entity(entity).despawn();
        }
    }
}

fn modify_spawn_settings(
    mut settings: ResMut<SpawnSettings>,
    player: Query<&Transform, With<Player>>,
) {
    let player_y = player.single().translation.y;

    // trace!("Player Y {player_y:?}");
    // higher number means more time between spawns
    if player_y > 300.0 {
        settings.lvl1_spawn = 0.08;
        settings.lvl2_spawn = 0.40;
    } else if player_y > 200.0 {
        settings.lvl1_spawn = 0.07;
    } else if player_y > 100.0 {
        settings.lvl1_spawn = 0.04;
    } else {
        settings.lvl1_spawn = 0.02;
    }
}
