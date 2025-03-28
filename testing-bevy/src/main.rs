mod commands;
mod input;

use bevy::color::palettes::css::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::pbr::light_consts::lux::FULL_DAYLIGHT;
use bevy::window::WindowMode;
use bevy::{core_pipeline::prepass::DepthPrepass, prelude::*};
use bevy_console::ConsolePlugin;
use bevy_panorbit_camera::PanOrbitCamera;
use bevy_rapier3d::prelude::*;
use std::f32::consts::PI;

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

const CAMERA_HEIGHT_OFFSET: f32 = 6.0;

const BOARD_WIDTH: f32 = 20.0;
const BOARD_HEIGHT: f32 = 40.0;
const BOARD_DEPTH: f32 = 1.25;

const BALL_BAR_FRICTION_RULE: Friction = Friction {
    coefficient: 0.10,
    combine_rule: CoefficientCombineRule::Average,
};

const STARTING_BAR_POS: Transform = Transform::from_xyz(-BOARD_WIDTH, -BOARD_HEIGHT / 2., 0.75);
const STARTING_BALL_POS: Transform = Transform::from_xyz(0.0, (-BOARD_HEIGHT / 2.) + 1.0, 0.0);
const BALL_RADIUS: f32 = 0.5;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let _camera = commands.spawn((
        PanOrbitCamera::default(),
        DepthPrepass,
        Transform::from_xyz(0.0, 0.5, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    let _directional_light = commands.spawn((
        DirectionalLight {
            illuminance: FULL_DAYLIGHT,
            ..Default::default()
        },
        Transform::from_xyz(0.0, 4.0, 18.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // TODO: understand depth mask to mask cylinder from wall, and add collider-only transparent
    // mesh to make ball fall off
    let _board = commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(BOARD_WIDTH, BOARD_HEIGHT, 0.5))),
        Transform::from_xyz(0.0, 0.0, -BOARD_DEPTH),
        Collider::cuboid(BOARD_WIDTH / 2., BOARD_HEIGHT / 2., 0.25),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: YELLOW.into(),
            ..Default::default()
        })),
        Wall,
    ));

    let _hole = commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(BALL_RADIUS + 0.1, BOARD_DEPTH + 0.10))),
        Transform::from_xyz(0.0, 5.0, -BOARD_DEPTH).with_rotation(Quat::from_rotation_x(PI / 2.)),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: BLACK.into(),
            ..Default::default()
        })),
        Hole,
    ));

    let _ball = commands.spawn((
        RigidBody::Dynamic,
        Mesh3d(
            meshes.add(
                Sphere {
                    radius: BALL_RADIUS,
                }
                .mesh(),
            ),
        ),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb_from_array([192., 189., 186.]),
            metallic: 0.6,
            perceptual_roughness: 0.1,
            ..Default::default()
        })),
        STARTING_BALL_POS,
        Collider::ball(0.5),
        BALL_BAR_FRICTION_RULE,
        ColliderMassProperties::Mass(100.0),
        Sleeping::disabled(),
        Ball,
    ));

    let bar = commands
        .spawn((
            RigidBody::Dynamic,
            Mesh3d(meshes.add(Cuboid::new(BOARD_WIDTH, 0.5, 0.5))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: SILVER.into(),
                ..Default::default()
            })),
            STARTING_BAR_POS,
            BALL_BAR_FRICTION_RULE,
            Collider::cuboid(BOARD_WIDTH / 2., 0.25, 0.25),
            Sleeping::disabled(),
            Bar,
        ))
        .id();

    let left_joint =
        RevoluteJointBuilder::new(Vec3::Z).local_anchor1(Vec3::new(-(BOARD_WIDTH / 2.), 0.0, 0.0));
    let _left_motor = commands.spawn((
        RigidBody::KinematicPositionBased,
        Transform::from_xyz(-BOARD_WIDTH / 2., 0.0, 0.0),
        ImpulseJoint::new(bar, left_joint),
        (Motor, Position::Left),
    ));

    let right_joint =
        RevoluteJointBuilder::new(Vec3::Z).local_anchor1(Vec3::new(BOARD_WIDTH / 2., 0.0, 0.0));
    let _right_motor = commands.spawn((
        RigidBody::KinematicPositionBased,
        Transform::from_xyz(BOARD_WIDTH / 2., 0.0, 0.0),
        ImpulseJoint::new(bar, right_joint),
        (Motor, Position::Right),
    ));
}

fn camera_follow_player(
    mut camera: Query<&mut Transform, (With<Camera3d>, Without<Ball>)>,
    ball: Query<&mut Transform, (With<Ball>, Without<Camera3d>)>,
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

    camera.translation = direction;
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(ConsolePlugin)
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (input::handle_inputs, camera_follow_player))
        .run();
}
