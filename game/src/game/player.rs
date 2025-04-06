use bevy::prelude::*;
use bevy_kenney_assets::KenneySpriteSheetAsset;

use crate::game::assets::{indices, ImageAssets};

#[derive(Component)]
pub struct Player;

pub fn setup(
    mut commands: Commands,
    asset_handles: Res<ImageAssets>,
    kenny_assets: Res<Assets<KenneySpriteSheetAsset>>,
    mut camera: Single<&mut Transform, With<Camera2d>>,
) {
    let space_sheet_asset = kenny_assets.get(&asset_handles.main_space_sheet).unwrap();
    let player_sprite = Sprite::from_atlas_image(
        space_sheet_asset.sheet.clone(),
        TextureAtlas {
            layout: space_sheet_asset.texture_atlas_layout.clone(),
            index: indices::sheet::PLAYERSHIP1_BLUE,
        },
    );
    info!("Entered game setup");
    commands.spawn((
        player_sprite,
        Transform {
            translation: Vec3::new(0.0, 0.0, 1.0),
            ..default()
        },
        Player,
    ));
    camera.scale = Vec3::new(5.0, 5.0, 1.0);
}

pub fn camera_movement(
    player: Single<&Transform, (With<Player>, Without<Camera>)>,
    mut camera: Single<&mut Transform, (With<Camera>, Without<Player>)>,
    time: Res<Time>,
) {
    const LERP_FACTOR: f32 = 2.0;

    let Vec3 { x, y, .. } = player.translation;
    let direction = Vec3::new(x, y, camera.translation.z);
    camera.translation = camera
        .translation
        .lerp(direction, time.delta_secs() * LERP_FACTOR);
}

pub fn movement(
    mut player: Query<&mut Transform, With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let move_speed = 400.0;
    let mut player_transform = player.single_mut();
    let mut translation = Vec2::ZERO;
    if input.pressed(KeyCode::ArrowLeft) {
        translation += Vec2::new(-1.0, 0.0);
    }

    if input.pressed(KeyCode::ArrowRight) {
        translation += Vec2::new(1.0, 0.0);
    }

    if input.pressed(KeyCode::ArrowDown) {
        translation += Vec2::new(0.0, -1.0);
    }

    if input.pressed(KeyCode::ArrowUp) {
        translation += Vec2::new(0.0, 1.0);
    }

    if translation != Vec2::ZERO {
        player_transform.translation +=
            translation.normalize().extend(0.0) * time.delta_secs() * move_speed;
        //player_transform.look_to(-translation.extend(1.0), Vec3::Z);
        player_transform.rotation = Quat::from_rotation_z(-translation.x.atan2(translation.y));
    }
}
