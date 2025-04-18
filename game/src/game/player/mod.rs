mod camera;
pub mod component;
mod movement;
pub mod setup;

use super::GameSystemSets::*;
use crate::GameState;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Game), setup::setup)
        .add_systems(
            FixedUpdate,
            (movement::up, camera::track_player).in_set(Pausable),
        );
}
