use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Component)]
struct Bar;

#[derive(Component)]
struct Ball;

fn setup(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(Camera3d::default())
        .insert(Transform::from_xyz(1.0, 2.0, 17.0).looking_at(Vec3::ZERO, Vec3::Y));
    commands
        .spawn(Collider::cuboid(8.0, 1.0, 1.0))
        .insert(Bar)
        .insert(MeshMaterial3d(materials.add(Color::WHITE)))
        .insert(Transform::from_xyz(0.0, 0.5, 0.0));
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Ball)
        .insert(MeshMaterial3d(materials.add(Color::WHITE)))
        .insert(Collider::ball(1.0))
        .insert(Transform::from_xyz(0.0, 4.0, 0.0));
}

//TODO:
fn move_bar() {}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup)
        .run();
}
