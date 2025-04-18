use std::f32::consts::PI;

use avian2d::prelude::{AngularDamping, Collider, ExternalForce, LinearDamping, Mass};
use bevy::prelude::*;
use bevy_enoki::ParticleSpawner;
use bevy_kenney_assets::KenneySpriteSheetAsset;

use super::component::{Player, PlayerBundle};
use crate::game::assets::{indices, ImageAssets};

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
    commands
        .spawn((
            PlayerBundle {
                sprite: player_sprite,
                collider: Collider::rectangle(200.0, 100.0),
                player: Player,
                ..default()
            },
            Mass(1.0),
            LinearDamping(1.0),
            AngularDamping(0.98),
            ExternalForce::ZERO.with_persistence(false),
            Transform::from_translation(Vec3::Z),
        ))
        .with_children(|cmd| {
            cmd.spawn((
                ParticleSpawner::default(),
                Transform::from_xyz(-50.0, 0.0, -0.1).with_rotation(Quat::from_rotation_z(PI)),
            ));
            cmd.spawn((
                ParticleSpawner::default(),
                Transform::from_xyz(50.0, 0.0, -0.1).with_rotation(Quat::from_rotation_z(PI)),
            ));
        });
    camera.scale = Vec3::new(5.0, 5.0, 1.0);
}
