use bevy::prelude::*;
use bevy_tiling_background::{
    BackgroundImageBundle, BackgroundMaterial, SetImageRepeatingExt, TilingBackgroundPlugin,
};

use crate::game::assets::ImageAssets;
use crate::GameState;

pub fn plugin(app: &mut App) {
    app.add_plugins(TilingBackgroundPlugin::<BackgroundMaterial>::default())
        .add_systems(OnEnter(GameState::Game), setup);
}

pub fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<BackgroundMaterial>>,
    image_assets: Res<ImageAssets>,
) {
    let image = image_assets.background.clone();
    commands.set_image_repeating(image.clone());
    commands.spawn((
        StateScoped(GameState::Game),
        BackgroundImageBundle::from_image(image, materials.as_mut())
            .with_movement_scale(0.1)
            .at_z_layer(0.1),
    ));
}
