use bevy::pbr::NotShadowCaster;
use bevy::prelude::*;

use crate::asset_tracking::LoadResource;

use super::theme;

const PLATFORM_HEGHT: f32 = 0.2;
const EPSILON: f32 = 0.001;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Platform>();

    app.register_type::<PlatformAssets>();
    app.load_resource::<PlatformAssets>();
}

fn arrow(
    size: Vec2,
    direction: Vec3,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    platform_assets: &PlatformAssets,
) -> impl Bundle {
    let arrow_rotation = Vec2::Y.angle_to(direction.zx());
    let mut arrow_transform =
        Transform::from_xyz(0.0, PLATFORM_HEGHT / 2.0 + EPSILON, 0.0).looking_to(-Vec3::Y, Vec3::Y);
    arrow_transform.rotate_y(arrow_rotation);

    (
        Name::new("Arrow"),
        arrow_transform,
        Mesh3d(meshes.add(Rectangle::new(size.x, size.y))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(platform_assets.arrow.clone()),
            alpha_mode: AlphaMode::Blend,
            ..Default::default()
        })),
        NotShadowCaster,
    )
}

/// The platform
pub fn platform(
    position: Vec2,
    size: Vec2,
    direction: Vec3,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    platform_assets: &PlatformAssets,
) -> impl Bundle {
    (
        Name::new("Platform"),
        Platform { direction },
        Transform::from_xyz(position.x, -PLATFORM_HEGHT / 2.0, position.y),
        Mesh3d(meshes.add(Cuboid::new(size.x, PLATFORM_HEGHT, size.y))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: theme::YELLOW,
            perceptual_roughness: 1.0,
            ..Default::default()
        })),
        NotShadowCaster,
        Children::spawn(Spawn(arrow(
            size,
            direction,
            meshes,
            materials,
            platform_assets,
        ))),
    )
}

#[derive(Component, Debug, Clone, Copy, Default, Reflect)]
#[reflect(Component)]
pub struct Platform {
    pub direction: Vec3,
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct PlatformAssets {
    #[dependency]
    arrow: Handle<Image>,
}

impl FromWorld for PlatformAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            arrow: assets.load("images/arrow.png"),
        }
    }
}
