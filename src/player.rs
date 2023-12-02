use crate::GameState;
use bevy::{
    input::mouse::MouseMotion,
    pbr::ShadowFilteringMethod,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use bevy_xpbd_3d::{math::Quaternion, prelude::*};

pub const ACCELERATION: f32 = 30.0;
pub const JUMP_VELOCITY: f32 = 10.0;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct CameraLook(Transform);

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(
                Update,
                (player_move, player_look, player_create_hook).run_if(in_state(GameState::Playing)),
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
                transform: Transform::from_xyz(0.0, 0.5, 0.0),
                ..default()
            },
            CameraLook(Transform::default()),
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            Collider::capsule(1.0, 0.5),
            LinearVelocity(Vec3::ZERO),
            GravityScale(2.0),
            LinearDamping(2.0),
            Player,
            ShapeCaster::new(
                Collider::cylinder(0.25, 0.5),
                Vec3::new(0.0, -1.0, 0.0),
                Quaternion::default(),
                Vec3::NEG_Y,
            )
            .with_max_time_of_impact(0.25),
        ))
        .with_children(|commands| {
            commands.spawn(Camera3dBundle {
                projection: Projection::Perspective(PerspectiveProjection {
                    fov: 90.0f32.to_radians(),
                    ..default()
                }),
                transform: Transform::from_xyz(0.0, 0.75, 0.0),
                ..default()
            });
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
        if key.just_pressed(KeyCode::Space) {
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
) {
    let mut window = window.single_mut();
    if mouse.just_pressed(MouseButton::Left) {
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;
    }
    if key.just_pressed(KeyCode::Escape) {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
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
    player: Query<(&Transform, &CameraLook, Entity), (With<Player>, Without<Camera3d>)>,
    entities: Query<&Transform, (Without<Player>, Without<Camera3d>, With<Collider>)>,
    caster: SpatialQuery,
    mouse: Res<Input<MouseButton>>,
    mut gizmos: Gizmos,
    mut commands: Commands,
) {
    let (player_transform, camera_look, player_entity) = player.single();

    if let Some(hit) = caster.cast_ray(
        player_transform.translation,
        camera_look.0.forward(),
        40.0,
        true,
        SpatialQueryFilter::new().without_entities([player_entity]),
    ) {
        let position = entities.get(hit.entity).unwrap();
        gizmos.sphere(position.translation, Quat::default(), 1.0, Color::WHITE);
        if mouse.just_pressed(MouseButton::Left) {}
    }
}
