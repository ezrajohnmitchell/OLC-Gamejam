use bevy::app::AppExit;
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::window::{PresentMode, WindowMode, WindowTheme};
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};
// use bevy_rapier2d::render::RapierDebugRenderPlugin;
use crate::spaceship::SpaceshipPlugin;
use crate::stars::StarsPlugin;

mod spaceship;
mod stars;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Space explorer".into(),
                present_mode: PresentMode::AutoNoVsync,
                mode: WindowMode::Windowed,
                window_theme: Some(WindowTheme::Dark),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(64.))
        // .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(StarsPlugin)
        .add_plugins(SpaceshipPlugin)
        .add_systems(Startup, setup_camera)
        .add_systems(Update, exit_system)
        .run();
}

#[derive(Component)]
pub struct CameraController;
fn setup_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle {
        camera: Camera {
            hdr: true,
            ..default()
        },
        tonemapping: Tonemapping::TonyMcMapface,
        transform: Transform::from_xyz(0., 0., 0.),
        ..default()
    };
    camera.projection.scaling_mode = ScalingMode::AutoMin { min_width: 800.0, min_height: 600.0 };

    commands.spawn((camera, BloomSettings::default(), CameraController));
}

fn exit_system(keys: Res<Input<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keys.pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}