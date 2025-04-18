mod camera;
pub mod component;
mod movement;
pub mod setup;

use bevy::prelude::*;

use crate::GameState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), setup::setup)
            .add_systems(Update, (movement::up).run_if(in_state(GameState::Game)))
            .add_systems(
                Update,
                (camera::track_player).run_if(in_state(GameState::Game)),
            );
    }
}
