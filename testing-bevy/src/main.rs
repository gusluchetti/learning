use bevy::{color::palettes::css::*, prelude::*};
use bevy_rapier3d::prelude::*;

#[derive(Component)]
struct Bar;

#[derive(Component)]
struct Ball;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(Camera3d::default())
        .insert(Transform::from_xyz(1.0, 2.0, 17.0).looking_at(Vec3::ZERO, Vec3::Y));

    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(0.0, 4.0, 18.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands
        .spawn(Mesh3d(meshes.add(Cuboid::new(1.0, 10.0, 1.0))))
        .insert(MeshMaterial3d(materials.add(StandardMaterial {
            base_color: RED.into(),
            ..Default::default()
        })))
        .insert(Collider::default())
        .insert(Transform::from_xyz(0.0, 0.5, 0.0))
        .insert(Bar);

    commands
        .spawn(Mesh3d(meshes.add(Sphere::default().mesh())))
        .insert(MeshMaterial3d(materials.add(StandardMaterial {
            base_color: RED.into(),
            ..Default::default()
        })))
        .insert(Collider::default())
        .insert(Transform::from_xyz(0.0, 6.0, 0.0))
        .insert(Ball);
}

// WIP: need to deal with other inputs
fn move_bar(kb_input: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Transform, With<Bar>>) {
    for mut transform in query.iter_mut() {
        if kb_input.pressed(KeyCode::ArrowUp) {
            transform.rotation.y += 2.0;
        }
        if kb_input.pressed(KeyCode::ArrowDown) {
            transform.rotation.y -= 2.0;
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, move_bar)
        .run();
}
