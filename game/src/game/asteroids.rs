use bevy::prelude::*;
use bevy_kenney_assets::KenneySpriteSheetAsset;

use super::assets::{indices, ImageAssets};
use crate::GameState;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Game), spawn);
}

#[derive(Component)]
struct Asteroid;

fn spawn(
    mut commands: Commands,
    handles: Res<ImageAssets>,
    assets: Res<Assets<KenneySpriteSheetAsset>>,
) {
    let space_assets = assets.get(&handles.main_space_sheet).unwrap();
    commands.spawn((
        Asteroid,
        Sprite::from_atlas_image(
            space_assets.sheet.clone(),
            TextureAtlas {
                layout: space_assets.texture_atlas_layout.clone(),
                index: indices::sheet::METEORBROWN_BIG1,
            },
        ),
        Transform {
            translation: Vec3::new(0.0, 0.0, 1.0),
            ..default()
        },
    ));
}
