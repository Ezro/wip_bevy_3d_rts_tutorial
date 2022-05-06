#![allow(clippy::redundant_field_names)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

use bevy::{prelude::*, render::{camera::ScalingMode, settings::WgpuSettings, render_resource::WgpuFeatures}, window::PresentMode, pbr::wireframe::WireframePlugin, winit::WinitSettings};
use bevy_inspector_egui::{WorldInspectorParams, WorldInspectorPlugin};
use bevy_editor_pls::*;
use bevy_obj::*;

pub const CLEAR: Color = Color::rgb(0.3, 0.3, 0.3);
pub const HEIGHT: f32 = 1080.0;
pub const RESOLUTION: f32 = 16.0 / 9.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WgpuSettings {
            features: WgpuFeatures::POLYGON_MODE_LINE,
            ..default()
        })
        .insert_resource(WindowDescriptor {
            width: HEIGHT * RESOLUTION,
            height: HEIGHT,
            title: "Bevy Template".to_string(),
            present_mode: PresentMode::Fifo,
            resizable: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .insert_resource(WinitSettings::desktop_app())
        .add_plugin(WireframePlugin)
        .add_plugin(EditorPlugin)
        .add_plugin(ObjPlugin)
        .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugin(bevy::diagnostic::EntityCountDiagnosticsPlugin)
        // .insert_resource(WorldInspectorParams {
        //     enabled: false,
        //     ..Default::default()
        // })
        // .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(startup_system)
        // .add_system(toggle_inspector)
        .run();
}

#[derive(Component)]
struct Cube;

fn startup_system(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut materials_asstes: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
){
    let mut cam = PerspectiveCameraBundle::new_3d();
    cam.transform.translation = Vec3::ONE * 50.0;
    cam.transform.look_at(Vec3::ZERO, Vec3::Y);
    commands.spawn_bundle(cam);
    commands.spawn_bundle(PbrBundle{
        mesh: mesh_assets.add(Mesh::from(shape::Plane { size: 8.0 })),
        material: materials_asstes.add(StandardMaterial {
            base_color: Color::rgb(1.0, 0.5, 0.5),
            ..Default::default()
        }),
        ..Default::default()
    });
    commands
        .spawn_bundle((
            Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                rotation: Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2),
                ..Default::default()
            },
            GlobalTransform::identity(),
        ))
        .with_children(|cell| {
            cell.spawn_scene(asset_server.load("models/free_car_001.glb#Scene0"));
            // cell.spawn_scene(asset_server.load("models/WoodenCabinBlender.glb#Scene0"));
        }
    );
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(0.0, 10.0, 0.0),
        point_light: PointLight {
            range: 30.0,
            ..Default::default()
        },
        ..default()
    });
}

#[allow(dead_code)]
fn slow_down() {
    std::thread::sleep(std::time::Duration::from_secs_f32(1.000));
}
