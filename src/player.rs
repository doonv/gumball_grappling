use crate::{hud::Score, materials::OutlineToonMaterial, GameState};
use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    input::mouse::MouseMotion,
    prelude::*,
    window::CursorGrabMode,
};
use bevy_atmosphere::plugin::AtmosphereCamera;
use bevy_toon_shader::ToonShaderMainCamera;
use bevy_xpbd_3d::{math::Quaternion, prelude::*};

pub const ACCELERATION: f32 = 30.0;
pub const JUMP_VELOCITY: f32 = 10.0;
pub const HOOK_SPEED: f32 = 0.75;
// pub const DASH_POWER: f32 = 50.0; // default
pub const DASH_POWER: f32 = 200.0;
pub const DASH_COOLDOWN: f64 = 1.0;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player {
    hooked_onto: Option<Entity>,
    dash: Option<()>,
}

#[derive(Component)]
pub struct CameraLook(Transform);

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(
                Update,
                (
                    player_move,
                    player_look,
                    player_create_hook,
                    player_use_and_remove_hook,
                    player_dash,
                    player_update_score,
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Capsule {
                    radius: 0.5,
                    depth: 1.0,
                    ..default()
                })),
                material: materials.add(Color::rgb_u8(124, 144, 255).into()),
                transform: Transform::from_xyz(0.0, 400.5, 0.0),
                ..default()
            },
            CameraLook(Transform::default()),
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            Collider::capsule(1.0, 0.5),
            LinearVelocity(Vec3::ZERO),
            GravityScale(20.0),
            LinearDamping(2.0),
            Player {
                hooked_onto: None,
                dash: Some(()),
            },
            ShapeCaster::new(
                Collider::cylinder(0.25, 0.5),
                Vec3::new(0.0, -1.0, 0.0),
                Quaternion::default(),
                Vec3::NEG_Y,
            )
            .with_max_time_of_impact(0.25),
        ))
        .with_children(|commands| {
            commands.spawn((
                Camera3dBundle {
                    camera: Camera {
                        hdr: true, // 1. HDR is required for bloom

                        ..default()
                    },
                    tonemapping: Tonemapping::SomewhatBoringDisplayTransform,
                    projection: Projection::Perspective(PerspectiveProjection {
                        fov: 90.0f32.to_radians(),
                        ..default()
                    }),
                    transform: Transform::from_xyz(0.0, 0.75, 0.0),
                    ..default()
                },
                ToonShaderMainCamera,
                AtmosphereCamera::default(),
                BloomSettings {
                    intensity: 0.3,
                    ..default()
                },
            ));
            // commands.spawn((
            //     ,
            //     TransformBundle::from_transform(Transform::from_xyz(0.0, -1.0, 0.0))
            // ));
        });
}

fn player_move(
    time: Res<Time>,
    key: Res<Input<KeyCode>>,
    mut player_query: Query<(&Transform, &mut LinearVelocity, &ShapeHits), With<Player>>,
) {
    for (transform, mut velocity, ground_caster_hits) in &mut player_query {
        let mut direction = Vec3::ZERO;
        if key.pressed(KeyCode::W) {
            direction += transform.forward();
        }
        if key.pressed(KeyCode::A) {
            direction += transform.left();
        }
        if key.pressed(KeyCode::S) {
            direction += transform.back();
        }
        if key.pressed(KeyCode::D) {
            direction += transform.right();
        }
        velocity.0 += direction * time.delta_seconds() * ACCELERATION;
        if key.just_pressed(KeyCode::Space) && !ground_caster_hits.is_empty() {
            velocity.0.y += JUMP_VELOCITY;
        }
    }
}
fn player_look(
    mut window: Query<&mut Window>,
    mut player: Query<(&mut Transform, &mut CameraLook), (With<Player>, Without<Camera3d>)>,
    mut camera: Query<&mut Transform, (Without<Player>, With<Camera3d>)>,
    mouse: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
    mut motion: EventReader<MouseMotion>,
    mut physics: ResMut<Time<Physics>>,
) {
    let mut window = window.single_mut();
    if mouse.just_pressed(MouseButton::Left) {
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;
        physics.unpause();
    }
    if key.just_pressed(KeyCode::Escape) {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
        physics.pause();
    }

    let (mut player_transform, mut camera_look) = player.single_mut();
    let mut camera_transform = camera.single_mut();

    if !window.cursor.visible {
        let motion = motion
            .read()
            .fold(Vec2::ZERO, |vec2, event| vec2 + event.delta);

        player_transform.rotate_y(motion.x * -0.001);
        camera_transform.rotate_x(motion.y * -0.001);

        camera_look.0.rotate_y(motion.x * -0.001);
        camera_look.0.rotate_local_x(motion.y * -0.001);
    }
}
fn player_create_hook(
    mut player: Query<
        (&mut Player, &Transform, &CameraLook, Entity),
        (With<Player>, Without<Camera3d>),
    >,
    entities: Query<
        &Handle<OutlineToonMaterial>,
        (Without<Player>, Without<Camera3d>, With<Collider>),
    >,
    caster: SpatialQuery,
    mouse: Res<Input<MouseButton>>,
    mut toon_materials: ResMut<Assets<OutlineToonMaterial>>,
    mut last_entity: Local<Option<Entity>>,
) {
    let (mut player, player_transform, camera_look, player_entity) = player.single_mut();

    if *last_entity != player.hooked_onto {
        if let Some(material_handle) = last_entity.map(|e| entities.get(e).ok()).flatten() {
            if let Some(material) = toon_materials.get_mut(material_handle) {
                material.outline_color = Color::NONE;
            }
        }
    }
    if let Some(hit) = caster.cast_ray(
        player_transform.translation,
        camera_look.0.forward(),
        40.0,
        true,
        SpatialQueryFilter::new().without_entities([player_entity]),
    ) {
        let Ok(material_handle) = entities.get(hit.entity) else {
            return;
        };
        *last_entity = Some(hit.entity);
        if let Some(material) = toon_materials.get_mut(material_handle) {
            material.outline_color = Color::rgb_linear(100.0, 100.0, 100.0);
        }
        if mouse.just_pressed(MouseButton::Left) {
            player.hooked_onto = Some(hit.entity);
            player.dash = Some(());
        }
    }
}
fn player_use_and_remove_hook(
    mut player: Query<(&mut Player, &mut LinearVelocity, &Transform, &CameraLook), With<Player>>,
    mut entities: Query<
        (
            &Transform,
            &Handle<OutlineToonMaterial>,
            &mut LinearVelocity,
        ),
        (Without<Player>, With<Collider>),
    >,
    mouse: Res<Input<MouseButton>>,
    mut gizmos: Gizmos,
    mut toon_materials: ResMut<Assets<OutlineToonMaterial>>,
) {
    if let Ok((mut player, mut velocity, transform, look)) = player.get_single_mut() {
        if let Some(hooked_onto) = player.hooked_onto {
            if let Ok((entity_transform, material_handle, mut other_velocity)) =
                entities.get_mut(hooked_onto)
            {
                gizmos.line(
                    transform.translation + look.0.forward() + Vec3::new(0.0, 0.75, 0.0),
                    entity_transform.translation,
                    Color::WHITE,
                );
                let direction = (entity_transform.translation - transform.translation).normalize();

                velocity.0 += direction * HOOK_SPEED;
                other_velocity.0 -= direction * HOOK_SPEED * 0.2;

                if mouse.just_released(MouseButton::Left) {
                    if let Some(material) = toon_materials.get_mut(material_handle) {
                        material.outline_color = Color::NONE;
                    }
                }
            }
            if mouse.just_released(MouseButton::Left) {
                player.hooked_onto = None;
            }
        }
    } else {
        error!("There is no player... wtf");
    }
}
fn player_dash(
    mut player: Query<(&mut LinearVelocity, &mut Player, &CameraLook), With<Player>>,
    mouse: Res<Input<MouseButton>>,
    mut last_dash_time: Local<f64>,
    time: Res<Time>,
) {
    if let Ok((mut velocity, mut player, look)) = player.get_single_mut() {
        let is_on_cooldown = (time.elapsed_seconds_f64() - *last_dash_time) < DASH_COOLDOWN;
        if mouse.just_pressed(MouseButton::Right) && player.dash.take().is_some() && !is_on_cooldown
        {
            velocity.0 += look.0.forward() * DASH_POWER;
            *last_dash_time = time.elapsed_seconds_f64();
        }
    }
}

fn player_update_score(player: Query<&Transform, With<Player>>, mut score: ResMut<Score>) {
    if let Ok(transform) = player.get_single() {
        // If `transform.translation.y` is out of range, this gives `u64::MAX` or `u64::MIN`
        score.0 = (transform.translation.y as u64 / 10).max(score.0)
    }
}
