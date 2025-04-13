use bevy::prelude::*;
use bevy_tiling_background::{
    BackgroundImageBundle, BackgroundMaterial, BackgroundMovementScale, SetImageRepeatingExt,
    TilingBackgroundPlugin,
};

use crate::GameState;

use crate::game::assets::ImageAssets;

pub fn plugin(app: &mut App) {
    app.add_plugins(TilingBackgroundPlugin::<BackgroundMaterial>::default())
        .add_systems(OnEnter(GameState::Game), setup)
        .add_systems(Update, movement.run_if(in_state(GameState::Game)))
        .add_systems(
            PostUpdate,
            update_movement_scale_system.run_if(in_state(GameState::Game)),
        );
}

pub fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<BackgroundMaterial>>,
    image_assets: Res<ImageAssets>,
) {
    let image = image_assets.background.clone();
    // Queue a command to set the image to be repeating once the image is loaded.
    commands.set_image_repeating(image.clone());

    commands.spawn(
        BackgroundImageBundle::from_image(image, materials.as_mut())
            .with_movement_scale(0.1)
            .at_z_layer(0.1),
    );
}

fn movement(
    mut background_scales: Query<&mut BackgroundMovementScale>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for mut background_scale in background_scales.iter_mut() {
        if input.pressed(KeyCode::KeyO) || input.pressed(KeyCode::KeyO) {
            background_scale.scale += time.delta_secs();
        }

        if input.pressed(KeyCode::KeyL) || input.pressed(KeyCode::KeyL) {
            background_scale.scale -= time.delta_secs();
        }
    }
}

pub fn update_movement_scale_system(
    query: Query<
        (
            &MeshMaterial2d<BackgroundMaterial>,
            &BackgroundMovementScale,
        ),
        Changed<BackgroundMovementScale>,
    >,
    mut background_materials: ResMut<Assets<BackgroundMaterial>>,
) {
    for (bg_material_handle, scale) in query.iter() {
        if let Some(background_material) = background_materials.get_mut(&**bg_material_handle) {
            background_material.movement_scale = scale.scale;
        }
    }
}
