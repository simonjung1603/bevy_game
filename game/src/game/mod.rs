use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::GameState;

mod assets;
mod asteroids;
mod background;
mod in_game_overlay;
mod player;

const TEXT_COLOR: Color = Color::srgb(0.5, 0.5, 1.0);

#[derive(Resource, Deref, DerefMut)]
struct Score(usize);

pub fn game_plugin(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(GameState::Loading)
            .continue_to_state(GameState::Game)
            .load_collection::<assets::AudioAssets>()
            .load_collection::<assets::ImageAssets>()
            .load_collection::<assets::FontAssets>(),
    )
    .add_plugins(background::plugin)
    .add_plugins((in_game_overlay::plugin, asteroids::plugin))
    .insert_resource(Score(0))
    .add_systems(OnEnter(GameState::Game), player::setup)
    .add_systems(Update, (player::movement).run_if(in_state(GameState::Game)))
    .add_systems(
        Update,
        player::camera_movement.run_if(in_state(GameState::Game)),
    );
}
