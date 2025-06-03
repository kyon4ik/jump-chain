//! Player-specific behavior.

use bevy::prelude::*;

use super::movement::{JumpSettings, JumpState};
use super::theme;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Player>();
}

/// The player character.
pub fn player(meshes: &mut Assets<Mesh>, materials: &mut Assets<StandardMaterial>) -> impl Bundle {
    (
        Name::new("Player"),
        Player,
        Transform::from_xyz(0.0, 0.8, 0.0),
        Mesh3d(meshes.add(Sphere::new(0.5).mesh().uv(32, 18))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: theme::GREEN,
            perceptual_roughness: 0.8,
            ..Default::default()
        })),
        JumpSettings {
            obj_height: 0.5,
            max_height: 1.0,
            flight_time: 1.0,
        },
        JumpState::default(),
    )
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
struct Player;

// fn record_player_directional_input(
//     input: Res<ButtonInput<KeyCode>>,
//     mut controller_query: Query<&mut MovementController, With<Player>>,
// ) {
//     // Collect directional input.
//     let mut intent = Vec2::ZERO;
//     if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
//         intent.y += 1.0;
//     }
//     if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
//         intent.y -= 1.0;
//     }
//     if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
//         intent.x -= 1.0;
//     }
//     if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
//         intent.x += 1.0;
//     }

//     // Normalize intent so that diagonal movement is the same speed as horizontal / vertical.
//     // This should be omitted if the input comes from an analog stick instead.
//     let intent = intent.normalize_or_zero();

//     // Apply movement intent to controllers.
//     for mut controller in &mut controller_query {
//         controller.intent = intent;
//     }
// }
