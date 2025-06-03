use bevy::prelude::*;

// mod animation;
pub mod level;
pub mod movement;
mod platform;
pub mod player;
pub mod theme;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        // animation::plugin,
        level::plugin,
        movement::plugin,
        platform::plugin,
        player::plugin,
    ));
}
