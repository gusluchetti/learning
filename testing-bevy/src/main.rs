use bevy::{color::palettes::css::*, prelude::*};
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

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let _camera = commands
        .spawn(Camera3d::default())
        .insert(Transform::from_xyz(1.0, 2.0, 17.0).looking_at(Vec3::ZERO, Vec3::Y));

    let _directional_light = commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(0.0, 4.0, 18.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    let _wall = commands
        .spawn(Mesh3d(meshes.add(Cuboid::new(30.0, 50.0, 0.5))))
        .insert(MeshMaterial3d(materials.add(StandardMaterial {
            base_color: YELLOW.into(),
            ..Default::default()
        })))
        .insert(Collider::cuboid(15.0, 25.0, 0.25))
        .insert(RigidBody::Fixed)
        .insert(Transform::from_xyz(0.0, 0.0, -1.1))
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

fn handle_bar_input(
    kb_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &Position), (With<Position>, With<Motor>)>,
) {
    //TODO: clamp Y diff between both motors
    const MOVE_SPEED: f32 = 0.025;
    for (mut transform, position) in query.iter_mut() {
        if let Position::Left = position {
            if kb_input.pressed(KeyCode::KeyW) {
                transform.translation.y += MOVE_SPEED;
            }
            if kb_input.pressed(KeyCode::KeyS) {
                transform.translation.y -= MOVE_SPEED;
            }
        }
        if let Position::Right = position {
            if kb_input.pressed(KeyCode::ArrowUp) {
                transform.translation.y += MOVE_SPEED;
            }
            if kb_input.pressed(KeyCode::ArrowDown) {
                transform.translation.y -= MOVE_SPEED;
            }
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, handle_bar_input)
        .run();
}
