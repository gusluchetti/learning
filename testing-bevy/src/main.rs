use bevy::color::palettes::css::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::pbr::light_consts::lux::OVERCAST_DAY;
use bevy::{core_pipeline::prepass::DepthPrepass, prelude::*};
use bevy_rapier3d::prelude::*;

#[derive(Component)]
struct Wall;

#[derive(Component)]
struct Motor;

#[derive(Component)]
enum Position {
    Left,
    Right,
}

#[derive(Component)]
struct Bar;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Hole;

const MOVE_SPEED: f32 = 0.015;
const MAX_DISTANCE: f32 = 3.0;
const CAMERA_HEIGHT_OFFSET: f32 = 4.0;
const HOLE_SIZE: f32 = 2.0;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let _camera = commands
        .spawn(Camera3d::default())
        .insert(DepthPrepass)
        .insert(Transform::from_xyz(-1.0, 1.0, 30.0).looking_at(Vec3::ZERO, Vec3::Y));

    let _directional_light = commands.spawn((
        DirectionalLight {
            illuminance: OVERCAST_DAY,
            ..Default::default()
        },
        Transform::from_xyz(0.0, 4.0, 18.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    let _hole = commands
        .spawn(Mesh3d(meshes.add(Cylinder::new(0.6, HOLE_SIZE))))
        .insert(MeshMaterial3d(materials.add(StandardMaterial {
            base_color: BLACK.into(),
            ..Default::default()
        })))
        .insert(
            Transform::from_xyz(5.0, 5.0, -HOLE_SIZE / 2.)
                .with_rotation(Quat::from_rotation_x(90.0)),
        )
        .insert(Hole);

    // TODO: understand depth mask to mask cylinder from wall, and add collider-only transparent
    // mesh to make ball fall off
    let _wall = commands
        .spawn(Mesh3d(meshes.add(Cuboid::new(30.0, 50.0, 0.5))))
        .insert(MeshMaterial3d(materials.add(StandardMaterial {
            base_color: YELLOW.into(),
            ..Default::default()
        })))
        .insert(Collider::cuboid(15.0, 25.0, 0.25))
        .insert(RigidBody::Fixed)
        .insert(Transform::from_xyz(0.0, 0.0, -1.2))
        .insert(Wall);

    let bar = commands
        .spawn(Mesh3d(meshes.add(Cuboid::new(20.0, 1.0, 1.0))))
        .insert(MeshMaterial3d(materials.add(StandardMaterial {
            base_color: SILVER.into(),
            ..Default::default()
        })))
        .insert(Collider::cuboid(10.0, 0.5, 0.5))
        .insert(RigidBody::Dynamic)
        .insert(Transform::from_xyz(0.0, 0.5, 0.0))
        .insert(Bar)
        .id();

    let left_joint = RevoluteJointBuilder::new(Vec3::Z).local_anchor1(Vec3::new(-6.0, 0.0, 0.0));
    let _left_motor = commands
        .spawn(RigidBody::KinematicPositionBased)
        .insert(Transform::from_xyz(-6.0, 0.0, 0.0))
        .insert(ImpulseJoint::new(bar, left_joint))
        .insert((Motor, Position::Left));

    let right_joint = RevoluteJointBuilder::new(Vec3::Z).local_anchor1(Vec3::new(6.0, 0.0, 0.0));
    let _right_motor = commands
        .spawn(RigidBody::KinematicPositionBased)
        .insert(Transform::from_xyz(6.0, 0.0, 0.0))
        .insert(ImpulseJoint::new(bar, right_joint))
        .insert((Motor, Position::Right));

    let _ball = commands
        .spawn(Mesh3d(meshes.add(Sphere::default().mesh())))
        .insert(MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb_from_array([192., 189., 186.]),
            metallic: 0.9,
            perceptual_roughness: 0.2,
            ..Default::default()
        })))
        .insert(Collider::ball(0.5))
        .insert(RigidBody::Dynamic)
        .insert(GravityScale(5.0))
        .insert(Transform::from_xyz(0.0, 6.0, 0.0))
        .insert(Ball);
}

fn camera_follow_player(
    mut camera: Query<&mut Transform, (With<Camera3d>, Without<Ball>)>,
    ball: Query<&mut Transform, (With<Ball>, Without<Camera3d>)>,
    time: Res<Time>,
) {
    let Ok(mut camera) = camera.get_single_mut() else {
        return;
    };

    let Ok(ball) = ball.get_single() else {
        return;
    };

    let Vec3 { x: _, y, z: _ } = ball.translation;
    let direction = Vec3::new(
        camera.translation.x,
        y + CAMERA_HEIGHT_OFFSET,
        camera.translation.z,
    );

    camera
        .translation
        .smooth_nudge(&direction, 0.1, time.delta_secs());
}

fn handle_bar_movement(
    mut motors: Query<(&mut Transform, &Position), (With<Motor>, With<Position>)>,
    kb_input: Res<ButtonInput<KeyCode>>,
) {
    let mut left_motor = None;
    let mut right_motor = None;

    for (transform, position) in motors.iter_mut() {
        match position {
            Position::Left => left_motor = Some((transform, position)),
            Position::Right => right_motor = Some((transform, position)),
        }
    }

    let left_translation_y = left_motor.expect("left exists").0.translation.y;
    let mut left_res: f32 = left_translation_y;
    if kb_input.pressed(KeyCode::KeyW) {
        left_res += MOVE_SPEED;
    }
    if kb_input.pressed(KeyCode::KeyS) {
        left_res -= MOVE_SPEED;
    }

    let right_translation_y = right_motor.expect("right exists").0.translation.y;
    let mut right_res: f32 = right_translation_y;
    if kb_input.pressed(KeyCode::ArrowUp) {
        right_res += MOVE_SPEED;
    }
    if kb_input.pressed(KeyCode::ArrowDown) {
        right_res -= MOVE_SPEED;
    }

    for (mut transform, position) in motors.iter_mut() {
        match position {
            Position::Left => {
                transform.translation.y =
                    left_res.clamp(right_res - MAX_DISTANCE, right_res + MAX_DISTANCE);
            }
            Position::Right => {
                transform.translation.y =
                    right_res.clamp(left_res - MAX_DISTANCE, left_res + MAX_DISTANCE);
            }
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (handle_bar_movement, camera_follow_player))
        .run();
}
