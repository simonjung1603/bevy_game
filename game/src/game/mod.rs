use bevy::{ecs::schedule::ScheduleLabel, prelude::*};
use bevy_asset_loader::prelude::*;
use GameSystemSets::*;

use crate::GameState;

mod assets;
pub mod asteroids;
mod background;
mod in_game_overlay;
mod particle_effects;
pub mod player;
mod weapons;

const TEXT_COLOR: Color = Color::srgb(0.5, 0.5, 1.0);

#[derive(SystemSet, Eq, PartialEq, Debug, Hash, Clone)]
pub enum GameSystemSets {
    OnlyRunInGame,
    Pausable,
    Mutable,
}

#[derive(Resource, PartialEq)]
struct Muted(bool);

pub fn game_plugin(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(GameState::Loading)
            .continue_to_state(GameState::Game)
            .load_collection::<assets::AudioAssets>()
            .load_collection::<assets::ImageAssets>()
            .load_collection::<assets::FontAssets>(),
    )
    // World
    .add_plugins((background::plugin, in_game_overlay::plugin))
    // Player
    .add_plugins(player::plugin)
    .add_plugins(weapons::plugin)
    // Enemies
    .add_plugins(asteroids::plugin)
    // Visual Effects
    .add_plugins(particle_effects::plugin)
    .insert_resource(Muted(false));

    configure_sets(app, Update);
    configure_sets(app, FixedUpdate);
}

fn configure_sets(app: &mut App, schedule: impl ScheduleLabel) {
    app.configure_sets(
        schedule,
        (
            Pausable.run_if(in_state(GameState::Game)),
            Mutable.run_if(resource_equals(Muted(false))),
        ),
    );
}
