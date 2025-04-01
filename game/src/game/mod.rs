use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::GameState;

mod scoreboard;

const TEXT_COLOR: Color = Color::srgb(0.5, 0.5, 1.0);

#[derive(AssetCollection, Resource)]
struct AudioAssets {
    #[asset(path = "audio/breakout_collision.ogg")]
    breakout: Handle<AudioSource>
}

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "images/playerShip1_red.png")]
    pub player: Handle<Image>,
    #[asset(path = "images/bg_black.png")]
    pub background: Handle<Image>,
}

#[derive(Resource, Deref, DerefMut)]
struct Score(usize);

pub fn game_plugin(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(GameState::Loading)
            .continue_to_state(GameState::Game)
            .load_collection::<AudioAssets>()
            .load_collection::<ImageAssets>(),
    )
    .insert_resource(Score(0))
    .add_systems(OnEnter(GameState::Game), (setup, scoreboard::setup))
    .add_systems(
        Update,
        scoreboard::update_scoreboard.run_if(in_state(GameState::Game)),
    );
}

#[derive(Component)]
struct Player;

fn setup(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
) {
    info!("Entered game setup");
    commands.spawn((
        Sprite::from_image(image_assets.player.clone()),
        Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3 { x: 0.5, y: 0.5, z: 1.0 },
            ..default()
        },
        Player,
    ));
}
