use bevy::prelude::*;

use crate::{AppSystems, PausableSystems};

use super::level::{GAP_SIZE, PLATFORM_SIZE};
use super::platform::Platform;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<JumpSettings>();
    app.register_type::<JumpState>();

    app.add_systems(
        Update,
        apply_jump
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct JumpSettings {
    pub obj_height: f32,
    pub max_height: f32,
    pub flight_time: f32,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct JumpState {
    acc_time: f32,
    coeffs: (f32, f32, f32),
    transformation: Transform,
}

pub fn init_jump_state(
    mut jump_query: Query<(&mut JumpState, &JumpSettings)>,
    mut ray_cast: MeshRayCast,
    platforms: Query<&Platform>,
) {
    let start = 0.0;
    // FIXME: Works only when platforms are squares
    let distance = (PLATFORM_SIZE + GAP_SIZE).x;
    let end = start + distance;

    for (mut jump_state, jump_settings) in &mut jump_query {
        let last_translation = Vec3::Y * jump_settings.obj_height;
        recalculate_jump_state(
            start,
            end,
            &mut jump_state,
            jump_settings,
            last_translation,
            &mut ray_cast,
            &platforms,
        );
    }
}

fn apply_jump(
    time: Res<Time>,
    #[cfg(feature = "dev")] mut gizmos: Gizmos,
    mut jump_query: Query<(&mut Transform, &mut JumpState, &JumpSettings)>,
    mut ray_cast: MeshRayCast,
    platforms: Query<&Platform>,
) {
    let start = 0.0;
    // FIXME: Works only when platforms are squares
    let distance = (PLATFORM_SIZE + GAP_SIZE).x;
    let end = start + distance;

    for (mut transform, mut jump_state, jump_settings) in &mut jump_query {
        jump_state.acc_time += time.delta_secs();

        if jump_state.acc_time >= jump_settings.flight_time {
            let last_translation = jump_state.transformation.translation
                + jump_state.transformation.rotation
                    * compute_jump(start, end, jump_state.coeffs, 1.0);

            jump_state.acc_time -= jump_settings.flight_time;
            recalculate_jump_state(
                start,
                end,
                &mut jump_state,
                jump_settings,
                last_translation,
                &mut ray_cast,
                &platforms,
            );
        }

        #[cfg(feature = "dev")]
        {
            gizmos.sphere(
                jump_state.transformation.to_isometry(),
                0.5,
                Color::srgb(0.0, 1.0, 0.0),
            );
            gizmos.arrow(
                jump_state.transformation.translation,
                jump_state.transformation.translation
                    + jump_state.transformation.forward().as_vec3(),
                Color::srgb(1.0, 0.0, 0.0),
            );
        }

        transform.translation = jump_state.transformation
            * compute_jump(
                start,
                end,
                jump_state.coeffs,
                jump_state.acc_time / jump_settings.flight_time,
            );
    }
}

#[inline]
fn compute_jump(start: f32, end: f32, coeffs: (f32, f32, f32), t: f32) -> Vec3 {
    let x = start.lerp(end, t);

    let (a, b, c) = coeffs;
    let y = a * x * x + b * x + c;
    let z = 0.0;

    // FIXME: hack to rotate using transform
    Vec3::new(z, y, -x)
}

fn recalculate_jump_state(
    start: f32,
    end: f32,
    jump_state: &mut JumpState,
    jump_settings: &JumpSettings,
    last_translation: Vec3,
    ray_cast: &mut MeshRayCast,
    platforms: &Query<&Platform>,
) {
    let a = -4.0 * jump_settings.max_height / (start - end) * (start - end);
    let b = -a * (start + end);
    let c = a * start * end;

    let ray = Ray3d::new(last_translation, Dir3::NEG_Y);
    let filter = |entity| platforms.contains(entity);
    let early_exit_test = |_entity| true;
    let visibility = RayCastVisibility::Visible;

    let settings = MeshRayCastSettings::default()
        .with_filter(&filter)
        .with_early_exit_test(&early_exit_test)
        .with_visibility(visibility);

    let hit = ray_cast.cast_ray(ray, &settings).first();
    if hit.is_none() {
        warn!("Did not hit platform with {:?} and {:?}?", ray, platforms);
    }
    let next_jump_direction = hit
        .map(|(entity, _)| platforms.get(*entity).unwrap().direction)
        .unwrap_or(Vec3::Z);

    jump_state.coeffs = (a, b, c);
    jump_state.transformation =
        Transform::from_translation(last_translation).looking_to(next_jump_direction, Vec3::Y);
}
