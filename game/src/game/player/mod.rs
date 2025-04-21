mod camera;
pub mod component;
pub mod experience;
mod movement;
pub mod setup;

use super::GameSystemSets::*;
use crate::GameState;
use bevy::prelude::*;
use experience::Experience;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Game), setup::setup)
        .add_systems(
            FixedUpdate,
            (
                movement::up,
                camera::track_player,
                experience::pickup::pickup_xp,
                experience::pickup::level_up.run_if(resource_changed::<Experience>),
            )
                .in_set(Pausable),
        );
}
