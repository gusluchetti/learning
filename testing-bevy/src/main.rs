use bevy::{color::palettes::css::*, core_pipeline::prepass::DepthPrepass, prelude::*};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
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

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // TODO: camera could slowly follow the ball? and should have an offset so it is slightly
    // higher than ball
    let _camera = commands
        .spawn(PanOrbitCamera::default())
        .insert(DepthPrepass)
        .insert(Transform::from_xyz(0.5, 1.0, 30.0).looking_at(Vec3::ZERO, Vec3::Y));

    let _directional_light = commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(0.0, 4.0, 18.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

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

    let _hole = commands
        .spawn(Mesh3d(meshes.add(Cylinder::new(0.6, 5.0))))
        .insert(MeshMaterial3d(materials.add(StandardMaterial {
            base_color: BLACK.into(),
            ..Default::default()
        })))
        .insert(Transform::from_xyz(0.0, 6.0, 0.0).rotate_y(90.0))
        .insert(Hole);

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

    let left_joint = SphericalJointBuilder::new().local_anchor1(Vec3::new(-6.0, 0.0, 0.0));
    let _left_motor = commands
        .spawn(RigidBody::KinematicPositionBased)
        .insert(Transform::from_xyz(-6.0, 0.0, 0.0))
        .insert(ImpulseJoint::new(bar, left_joint))
        .insert((Motor, Position::Left));

    let right_joint = SphericalJointBuilder::new().local_anchor1(Vec3::new(6.0, 0.0, 0.0));
    let _right_motor = commands
        .spawn(RigidBody::KinematicPositionBased)
        .insert(Transform::from_xyz(6.0, 0.0, 0.0))
        .insert(ImpulseJoint::new(bar, right_joint))
        .insert((Motor, Position::Right));

    let _ball = commands
        .spawn(Mesh3d(meshes.add(Sphere::default().mesh())))
        .insert(MeshMaterial3d(materials.add(StandardMaterial {
            base_color: GRAY.into(),
            ..Default::default()
        })))
        .insert(Collider::ball(0.5))
        .insert(RigidBody::Dynamic)
        .insert(ColliderMassProperties::Density(6.0))
        .insert(Transform::from_xyz(0.0, 6.0, 0.0))
        .insert(Ball);
}

fn handle_bar_movement(
    kb_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &Position), (With<Motor>, With<Position>)>,
) {
    const MOVE_SPEED: f32 = 0.025;
    const MAX_DISTANCE: f32 = 4.0;

    let mut left_motor = None;
    let mut right_motor = None;

    for (transform, position) in query.iter_mut() {
        match position {
            Position::Left => left_motor = Some((transform, position)),
            Position::Right => right_motor = Some((transform, position)),
        }
    }

    let left_translation_y = left_motor.expect("left exists").0.translation.y;
    let right_translation_y = right_motor.expect("right exists").0.translation.y;

    let mut left_res: f32 = 0.0;
    if kb_input.pressed(KeyCode::KeyW) {
        left_res = left_translation_y + MOVE_SPEED;
    }
    if kb_input.pressed(KeyCode::KeyS) {
        left_res = left_translation_y - MOVE_SPEED;
    }

    let mut right_res: f32 = 0.0;
    if kb_input.pressed(KeyCode::ArrowUp) {
        right_res = right_translation_y + MOVE_SPEED;
    }
    if kb_input.pressed(KeyCode::ArrowDown) {
        right_res = right_translation_y - MOVE_SPEED;
    }

    for (mut transform, position) in query.iter_mut() {
        match position {
            Position::Left => {
                transform.translation.y =
                    ((left_res - right_translation_y).abs()).clamp(-1., MAX_DISTANCE);
            }
            Position::Right => {
                transform.translation.y =
                    ((right_res - left_translation_y).abs()).clamp(-1., MAX_DISTANCE);
            }
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, handle_bar_movement)
        .run();
}
