use std::time::Duration;

use avian2d::prelude::{Collider, CollidingEntities, Collision, LinearVelocity, RigidBody, Sensor};
use bevy::{math::VectorSpace, prelude::*, time::common_conditions::on_timer};
use bevy_kenney_assets::KenneySpriteSheetAsset;

use crate::GameState;

use super::{
    assets::{indices, AudioAssets, ImageAssets},
    asteroids::Asteroid,
    player::component::Player,
};

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(GameState::Game),
        add_pulse_weapon.after(super::player::setup::setup),
    )
    .add_systems(
        Update,
        fire.run_if(in_state(GameState::Game).and(on_timer(Duration::from_secs(1)))),
    )
    .add_systems(Update, tick_lifetime.run_if(in_state(GameState::Game)));
}

fn add_pulse_weapon(mut cmds: Commands, player: Single<Entity, Added<Player>>) {
    cmds.get_entity(*player).unwrap().with_child((
        Transform::from_translation(Vec3::ZERO),
        Collider::triangle(
            Vec2::ZERO,
            Vec2::new(1000.0, 2000.0),
            Vec2::new(-1000.0, 2000.0),
        ),
        Sensor,
        CollidingEntities::default(),
        PulseLaser::default(),
    ));
}

#[derive(Component, Deref, DerefMut)]
struct Lifetime(Timer);

fn fire(
    mut cmds: Commands,
    aim_sector: Single<(&GlobalTransform, &CollidingEntities), With<PulseLaser>>,
    asset_handles: Res<ImageAssets>,
    audio_handles: Res<AudioAssets>,
    kenny_assets: Res<Assets<KenneySpriteSheetAsset>>,
    asteroids: Query<&Transform, With<Asteroid>>,
) {
    let space_sheet_asset = kenny_assets.get(&asset_handles.main_space_sheet).unwrap();
    let laser_pulse_sprite = Sprite::from_atlas_image(
        space_sheet_asset.sheet.clone(),
        TextureAtlas {
            layout: space_sheet_asset.texture_atlas_layout.clone(),
            index: indices::sheet::BEAM0,
        },
    );

    let (spawn_position, targetable_entities) = aim_sector.into_inner();

    let closest_enemy = targetable_entities
        .0
        .iter()
        .filter_map(|colliding_entity| asteroids.get(*colliding_entity).ok())
        .min_by(|a, b| {
            a.translation
                .distance(spawn_position.translation())
                .total_cmp(&b.translation.distance(spawn_position.translation()))
        });

    let transform = if let Some(enemy) = closest_enemy {
        Transform {
            translation: spawn_position.translation(),
            rotation: spawn_position.rotation()
                * Quat::from_rotation_arc(
                    spawn_position.up().as_vec3(),
                    (enemy.translation - spawn_position.translation()).normalize(),
                ),
            scale: spawn_position.scale(),
        }
    } else {
        Transform {
            translation: spawn_position.translation(),
            rotation: spawn_position.rotation(),
            scale: spawn_position.scale(),
        }
    };

    cmds.spawn((
        transform,
        RigidBody::Dynamic,
        Lifetime(Timer::from_seconds(0.8, TimerMode::Once)),
        AudioPlayer::new(audio_handles.laser.clone()),
        LinearVelocity((transform.rotation * Vec2::Y.extend(0.0)).xy() * 5000.0),
        laser_pulse_sprite,
    ));
}

fn tick_lifetime(
    mut cmds: Commands,
    mut entities: Query<(Entity, &mut Lifetime)>,
    time: Res<Time>,
) {
    for (entity, mut lifetime) in &mut entities {
        if lifetime.tick(time.delta()).finished() {
            cmds.get_entity(entity).unwrap().despawn_recursive();
        }
    }
}

#[derive(Component, Default, Reflect)]
pub struct PulseLaser {
    pub sprite_id: usize,
    pub cooldown: f32,
    pub reload_duration: f32,
    pub duration: f32,
}
