use std::f32::consts::{PI, TAU};

use bevy::prelude::*;
use bevy_kenney_assets::KenneySpriteSheetAsset;

use super::assets::{indices, ImageAssets};
use crate::GameState;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Game), spawn)
        .add_systems(Update, continuos_rotation.run_if(in_state(GameState::Game)));
}

#[derive(Component)]
struct Asteroid;

#[derive(Component, Deref, DerefMut)]
struct RotateContinuos(f32);

fn spawn(
    mut commands: Commands,
    handles: Res<ImageAssets>,
    assets: Res<Assets<KenneySpriteSheetAsset>>,
    window: Single<&Window>,
    camera: Single<&Transform, With<Camera2d>>,
) {
    let space_assets = assets.get(&handles.main_space_sheet).unwrap();

    // Screen bounds for spawning asteroids
    let scale = camera.scale.z * 2.0;
    let (width, height) = (window.width() * scale, window.height() * scale);
    let x_range = -width..width;
    let y_range = -height..height;

    let asteroid_indices = [
        indices::sheet::METEORBROWN_BIG1,
        indices::sheet::METEORBROWN_BIG2,
        indices::sheet::METEORBROWN_BIG3,
        indices::sheet::METEORBROWN_BIG4,
        indices::sheet::METEORGREY_BIG1,
        indices::sheet::METEORGREY_BIG2,
        indices::sheet::METEORGREY_BIG3,
        indices::sheet::METEORGREY_BIG4,
    ];

    for _ in 0..20 {
        commands.spawn((
            Asteroid,
            Sprite::from_atlas_image(
                space_assets.sheet.clone(),
                TextureAtlas {
                    layout: space_assets.texture_atlas_layout.clone(),
                    index: asteroid_indices[rand::random_range(0..asteroid_indices.len())],
                },
            ),
            Transform {
                translation: Vec3::new(
                    rand::random_range(x_range.clone()),
                    rand::random_range(y_range.clone()),
                    1.0,
                ),
                ..default()
            },
            RotateContinuos(
                if rand::random_bool(0.5) { -1.0 } else { 1.0 } * rand::random_range(1.0..TAU),
            ),
        ));
    }
}

fn continuos_rotation(mut transforms: Query<(&mut Transform, &RotateContinuos)>, time: Res<Time>) {
    for (mut transform, rotation_per_second) in &mut transforms {
        transform.rotate_z(rotation_per_second.0 * time.delta_secs());
    }
}
