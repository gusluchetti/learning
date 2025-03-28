mod commands;
mod input;

use bevy::color::palettes::css::*;
use bevy::core_pipeline::prepass::{MotionVectorPrepass, NormalPrepass};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::pbr::light_consts::lux::FULL_DAYLIGHT;
use bevy::render::render_resource::{AsBindGroup, ShaderRef, ShaderType};
use bevy::window::WindowMode;
use bevy::{core_pipeline::prepass::DepthPrepass, prelude::*};
use bevy_console::{AddConsoleCommand, ConsolePlugin};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use bevy_rapier3d::prelude::*;
use commands::{ExampleCommand, example_command};
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(MaterialPlugin::<PrepassOutputMaterial> {
            prepass_enabled: false,
            ..default()
        })
        .add_plugins(ConsolePlugin)
        .add_console_command::<ExampleCommand, _>(example_command)
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                input::handle_inputs,
                camera_follow_player,
                toggle_prepass_view,
            ),
        )
        .run();
}

#[derive(Component)]
struct Board;

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

#[derive(Debug, Clone, Default, ShaderType)]
struct ShowPrepassSettings {
    show_depth: u32,
    show_normals: u32,
    show_motion_vectors: u32,
    padding_1: u32,
    padding_2: u32,
}

// This shader simply loads the prepass texture and outputs it directly
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct PrepassOutputMaterial {
    #[uniform(0)]
    settings: ShowPrepassSettings,
}

impl Material for PrepassOutputMaterial {
    fn fragment_shader() -> ShaderRef {
        PREPASS_SHADER_ASSET_PATH.into()
    }

    // This needs to be transparent in order to show the scene behind the mesh
    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

const PREPASS_SHADER_ASSET_PATH: &str = "shaders/show_prepass.wgsl";

const INIT_CAMERA_POS: Transform = Transform::from_xyz(0., (-BOARD_HEIGHT / 2.) + 3., 20.);
const INIT_BALL_POS: Transform = Transform::from_xyz(0., 2., 0.);
const INIT_BAR_POS: Transform = Transform::from_xyz(0., 1., -BOARD_WIDTH + 0.01);
const LEFT_JOINT_POS: Transform = Transform::from_xyz(
    -(BOARD_WIDTH / 2.),
    INIT_BAR_POS.translation.y,
    INIT_BAR_POS.translation.z,
);
const RIGHT_JOINT_POS: Transform = Transform::from_xyz(
    BOARD_WIDTH / 2.,
    INIT_BAR_POS.translation.y,
    INIT_BAR_POS.translation.z,
);

const CAMERA_HEIGHT_OFFSET: f32 = 6.0;

const BOARD_WIDTH: f32 = 20.0;
const BOARD_HEIGHT: f32 = 40.0;
const BOARD_DEPTH: f32 = 1.25;

const BALL_RADIUS: f32 = 0.3;

const BALL_BAR_FRICTION_RULE: Friction = Friction {
    coefficient: 0.25,
    combine_rule: CoefficientCombineRule::Average,
};

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut depth_materials: ResMut<Assets<PrepassOutputMaterial>>,
) {
    let _camera = commands.spawn((
        PanOrbitCamera::default(),
        DepthPrepass,
        NormalPrepass,
        MotionVectorPrepass,
        INIT_CAMERA_POS.looking_at(INIT_BALL_POS.translation, Vec3::Y),
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
        Board,
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
        // MeshMaterial3d(materials.add(StandardMaterial {
        //     base_color: Color::srgb_from_array([192., 189., 186.]),
        //     metallic: 0.6,
        //     perceptual_roughness: 0.1,
        //     ..Default::default()
        // })),
        MeshMaterial3d(depth_materials.add(PrepassOutputMaterial {
            settings: ShowPrepassSettings::default(),
        })),
        INIT_BALL_POS,
        Collider::ball(BALL_RADIUS),
        BALL_BAR_FRICTION_RULE,
        GravityScale(2.),
        ColliderMassProperties::Density(10.0),
        Velocity::default(),
        Sleeping::disabled(),
        Ball,
    ));

    let bar = commands
        .spawn((
            RigidBody::Dynamic,
            Mesh3d(meshes.add(Cuboid::new(BOARD_WIDTH, 0.5, 0.75))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: SILVER.into(),
                ..Default::default()
            })),
            INIT_BAR_POS,
            BALL_BAR_FRICTION_RULE,
            Collider::cuboid(BOARD_WIDTH / 2., 0.25, 0.375),
            Sleeping::disabled(),
            Bar,
        ))
        .id();

    let left_joint = RevoluteJointBuilder::new(Vec3::Z).local_anchor1(LEFT_JOINT_POS.translation);
    let _left_motor = commands.spawn((
        RigidBody::KinematicPositionBased,
        LEFT_JOINT_POS,
        ImpulseJoint::new(bar, left_joint),
        (Motor, Position::Left),
    ));

    let right_joint = RevoluteJointBuilder::new(Vec3::Z).local_anchor1(RIGHT_JOINT_POS.translation);
    let _right_motor = commands.spawn((
        RigidBody::KinematicPositionBased,
        RIGHT_JOINT_POS,
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

/// Every time you press space, it will cycle between transparent, depth and normals view
fn toggle_prepass_view(
    mut prepass_view: Local<u32>,
    keycode: Res<ButtonInput<KeyCode>>,
    material_handle: Single<&MeshMaterial3d<PrepassOutputMaterial>>,
    mut materials: ResMut<Assets<PrepassOutputMaterial>>,
    text: Single<Entity, With<Text>>,
    mut writer: TextUiWriter,
) {
    if keycode.just_pressed(KeyCode::Space) {
        *prepass_view = (*prepass_view + 1) % 4;

        let label = match *prepass_view {
            0 => "transparent",
            1 => "depth",
            2 => "normals",
            3 => "motion vectors",
            _ => unreachable!(),
        };
        let text = *text;
        *writer.text(text, 1) = format!("Prepass Output: {label}\n");
        writer.for_each_color(text, |mut color| {
            color.0 = Color::WHITE;
        });

        let mat = materials.get_mut(*material_handle).unwrap();
        mat.settings.show_depth = (*prepass_view == 1) as u32;
        mat.settings.show_normals = (*prepass_view == 2) as u32;
        mat.settings.show_motion_vectors = (*prepass_view == 3) as u32;
    }
}
