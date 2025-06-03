//! Spawn the main level.

use bevy::ecs::spawn::SpawnIter;
use bevy::prelude::*;
use rand::seq::SliceRandom;

use crate::asset_tracking::LoadResource;
use crate::audio::music;
use crate::demo::platform::platform;
use crate::demo::player::player;
use crate::screens::Screen;

use super::platform::PlatformAssets;

pub const GAP_SIZE: Vec2 = Vec2::splat(0.1);
pub const PLATFORM_COUNT: UVec2 = UVec2::splat(5);
pub const PLATFORM_SIZE: Vec2 = Vec2::splat(1.0);

pub(super) fn plugin(app: &mut App) {
    app.register_type::<LevelAssets>();
    app.load_resource::<LevelAssets>();
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct LevelAssets {
    #[dependency]
    music: Handle<AudioSource>,
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            music: assets.load("audio/music/longnight.ogg"),
        }
    }
}

/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    platform_assets: Res<PlatformAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Name::new("Level"),
        Transform::default(),
        Visibility::default(),
        StateScoped(Screen::Gameplay),
        children![
            player(&mut meshes, &mut materials),
            (
                Name::new("Gameplay Music"),
                music(level_assets.music.clone())
            )
        ],
    ));

    const JUMP_DIRECTIONS: &[Vec3] = &[Vec3::X, Vec3::NEG_X, Vec3::Z, Vec3::NEG_Z];
    let rng = &mut rand::thread_rng();

    let mut platforms = Vec::new();
    for x in 0..PLATFORM_COUNT.x {
        for y in 0..PLATFORM_COUNT.y {
            let mut platform_pos =
                -(PLATFORM_COUNT.as_vec2() * (PLATFORM_SIZE + GAP_SIZE) - GAP_SIZE) / 2.0;
            platform_pos +=
                UVec2::new(x, y).as_vec2() * (PLATFORM_SIZE + GAP_SIZE) + PLATFORM_SIZE / 2.0;
            platforms.push(platform(
                platform_pos,
                PLATFORM_SIZE,
                *JUMP_DIRECTIONS.choose(rng).unwrap(),
                &mut meshes,
                &mut materials,
                &platform_assets,
            ));
        }
    }

    commands.spawn((
        Name::new("Platforms"),
        Transform::default(),
        Visibility::default(),
        StateScoped(Screen::Gameplay),
        Children::spawn(SpawnIter(platforms.into_iter())),
    ));
}
