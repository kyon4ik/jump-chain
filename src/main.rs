// Support configuring Bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

mod asset_tracking;
mod audio;
mod demo;
#[cfg(feature = "dev")]
mod dev_tools;
mod menus;
mod screens;
mod theme;

use bevy::asset::AssetMetaCheck;
use bevy::core_pipeline::bloom::Bloom;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::window::WindowMode;

fn main() -> AppExit {
    App::new().add_plugins(AppPlugin).run()
}

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Add Bevy plugins.
        app.add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics on web build on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "Jump Chain".to_string(),
                        fit_canvas_to_parent: true,
                        #[cfg(feature = "dev")]
                        present_mode: bevy::window::PresentMode::AutoNoVsync,
                        mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                        resizable: false,
                        ..default()
                    }
                    .into(),
                    ..default()
                }),
        );

        // Add other plugins.
        app.add_plugins((
            asset_tracking::plugin,
            audio::plugin,
            demo::plugin,
            #[cfg(feature = "dev")]
            dev_tools::plugin,
            menus::plugin,
            screens::plugin,
            theme::plugin,
        ));

        // Order new `AppSystems` variants by adding them here:
        app.configure_sets(
            Update,
            (
                AppSystems::TickTimers,
                AppSystems::RecordInput,
                AppSystems::Update,
            )
                .chain(),
        );

        // Set up the `Pause` state.
        app.init_state::<Pause>();
        app.configure_sets(Update, PausableSystems.run_if(in_state(Pause(false))));

        // Spawn the main camera.
        app.add_systems(Startup, (spawn_lights, spawn_camera).chain());
    }
}

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum AppSystems {
    /// Tick timers.
    TickTimers,
    /// Record player input.
    RecordInput,
    /// Do everything else (consider splitting this into further variants).
    Update,
}

/// Whether or not the game is paused.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[states(scoped_entities)]
struct Pause(pub bool);

/// A system set for systems that shouldn't run while the game is paused.
#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct PausableSystems;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera3d::default(),
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 10.0,
            },
            scale: 2.0,
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_xyz(5.0, 7.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        Camera {
            hdr: true,
            clear_color: ClearColorConfig::Custom(demo::theme::DARK_BLUE),
            ..Default::default()
        },
        #[cfg(not(target_family = "wasm"))]
        Msaa::Sample4,
        Tonemapping::TonyMcMapface,
        Bloom::NATURAL,
    ));
}

fn spawn_lights(mut commands: Commands) {
    // commands.insert_resource(AmbientLight {
    //     color: demo::theme::THEME_YELLOW,
    //     brightness: 80.0,
    //     ..Default::default()
    // });

    commands.spawn((
        Name::new("Directional Light"),
        DirectionalLight {
            illuminance: light_consts::lux::AMBIENT_DAYLIGHT,
            shadows_enabled: true,
            ..Default::default()
        },
        Transform::from_xyz(0.0, 10.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
