#![windows_subsystem = "windows"]

use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
    render::camera::ScalingMode,
    render::settings::{WgpuSettings, WgpuSettingsPriority},
    //?? winit::WinitSettings,
    window::PresentMode,
};

// use bevy_framepace;
// use bevy_pkv::PkvStore;

mod background;
mod game_controller;
mod options;
mod pipes;
mod player;
mod sound;
mod ui;
mod window;
mod android;

use background::BackgroundPlugin;
use game_controller::*;
use options::*;
use pipes::*;
use player::*;


#[bevy_main]
fn main() {
    App::new()
        // #[cfg(target_arch="aarch64-linux-android", "armv7-linux-androideabi")]
        .insert_resource(WgpuSettings {
            priority: WgpuSettingsPriority::Compatibility,
            ..default()
        })
        // .insert_resource(WinitSettings::desktop_app()) //? this breaks the game??
        .insert_resource(ClearColor(Color::rgb(
            BACKGROUND_COLOR[0] / 255.0,
            BACKGROUND_COLOR[1] / 255.0,
            BACKGROUND_COLOR[2] / 255.0,
        )))
        // Default plugins
        .add_plugin(PlatformBuildPlugin)
        // .add_plugins(DefaultPlugins)
        // // PKV data storage
        // .insert_resource(PkvStore::new("bewuwy", GAME_NAME))
        // Audio
        .add_plugin(sound::SoundPlugin)
        // FPS
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // Game systems
        .init_resource::<GameController>()
        .add_startup_system(setup)
        .add_plugin(PipesPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(BackgroundPlugin)
        // UI
        .add_plugin(ui::UIPlugin)
        // // Window
        // .add_plugin(window::WindowIconPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    // Add a 2D Camera
    commands.spawn_bundle(Camera2dBundle {
        projection: OrthographicProjection {
            // far: 1000.0,
            scaling_mode: ScalingMode::FixedVertical(WINDOW_HEIGHT),
            ..Default::default()
        },
        ..Default::default()
    });
}

struct PlatformBuildPlugin;

impl Plugin for PlatformBuildPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(not(target_arch="aarch64-linux-android"))]
        #[cfg(not(target_arch="armv7-linux-androideabi"))]
        app.insert_resource(WindowDescriptor {
            title: GAME_NAME.to_string(),
            // resizable: false,
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            present_mode: PresentMode::AutoVsync,
            ..default()
        });

        if cfg!(target_arch="aarch64-linux-android") || cfg!(target_arch="armv7-linux-androideabi") {
            app.add_plugins(android::DefaultAndroidPlugins);
        }
        else {
            app.add_plugins(DefaultPlugins);
        }
    }
}
