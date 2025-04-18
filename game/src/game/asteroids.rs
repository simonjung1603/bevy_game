use std::f32::consts::{PI, TAU};

use avian2d::prelude::{AngularVelocity, Collider, LinearVelocity, RigidBody};
use bevy::prelude::*;
use bevy_kenney_assets::KenneySpriteSheetAsset;

use super::assets::{indices, ImageAssets};
use crate::GameState;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Game), setup_wave_system)
        .add_systems(
            Update,
            (spawn_wave, check_asteroid_bounds).run_if(in_state(GameState::Game)),
        );
}

#[derive(Component)]
pub struct Asteroid;

#[derive(Resource)]
struct WaveSystem {
    current_wave: u32,
    asteroids_per_wave: u32,
    spawn_timer: Timer,
    spawn_interval: f32,
    base_spawn_interval: f32,
    min_spawn_interval: f32,
    spawn_interval_decrease: f32,
    wave_timer: Timer,
}

impl Default for WaveSystem {
    fn default() -> Self {
        Self {
            current_wave: 0,
            asteroids_per_wave: 5,
            spawn_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
            spawn_interval: 2.0,
            base_spawn_interval: 2.0,
            min_spawn_interval: 0.5,
            spawn_interval_decrease: 0.1,
            wave_timer: Timer::from_seconds(30.0, TimerMode::Repeating),
        }
    }
}

fn setup_wave_system(mut commands: Commands) {
    commands.insert_resource(WaveSystem::default());
}

fn spawn_wave(
    mut commands: Commands,
    mut wave_system: ResMut<WaveSystem>,
    handles: Res<ImageAssets>,
    assets: Res<Assets<KenneySpriteSheetAsset>>,
    window: Single<&Window>,
    camera: Single<&Transform, With<Camera2d>>,
    time: Res<Time>,
) {
    // Update timers
    wave_system.spawn_timer.tick(time.delta());
    wave_system.wave_timer.tick(time.delta());

    // Check if it's time to advance to the next wave
    if wave_system.wave_timer.just_finished() {
        wave_system.current_wave += 1;
        wave_system.asteroids_per_wave += 2; // Increase asteroids per wave
        wave_system.spawn_interval = (wave_system.base_spawn_interval
            - (wave_system.current_wave as f32 * wave_system.spawn_interval_decrease))
            .max(wave_system.min_spawn_interval);
        wave_system.spawn_timer =
            Timer::from_seconds(wave_system.spawn_interval, TimerMode::Repeating);
        info!(
            "Wave {} started! Spawning {} asteroids",
            wave_system.current_wave, wave_system.asteroids_per_wave
        );
    }

    // Spawn asteroids if the timer is finished
    if wave_system.spawn_timer.just_finished() {
        let space_assets = assets.get(&handles.main_space_sheet).unwrap();

        // Screen bounds for spawning asteroids
        // The camera scale is set to 5.0 in player.rs, which means the camera is zoomed out
        // We need to account for this to properly position asteroids off-screen
        // Calculate the visible area of the world
        let visible_width = window.width() * camera.scale.x;
        let visible_height = window.height() * camera.scale.y;
        let screen_center_x = camera.translation.x;
        let screen_center_y = camera.translation.y;

        // Add a margin to ensure asteroids are fully off-screen
        let margin = 200.0;

        // Spawn asteroids at the edges of the screen
        let spawn_side = rand::random_range(0..4);
        let (x, y) = match spawn_side {
            0 => (
                screen_center_x + visible_width / 2.0 + margin,
                screen_center_y + rand::random_range(-visible_height / 2.0..visible_height / 2.0),
            ), // Right
            1 => (
                screen_center_x + -visible_width / 2.0 - margin,
                screen_center_y + rand::random_range(-visible_height / 2.0..visible_height / 2.0),
            ), // Left
            2 => (
                screen_center_x + rand::random_range(-visible_width / 2.0..visible_width / 2.0),
                screen_center_y + visible_height / 2.0 + margin,
            ), // Top
            _ => (
                screen_center_x + rand::random_range(-visible_width / 2.0..visible_width / 2.0),
                screen_center_y + -visible_height / 2.0 - margin,
            ), // Bottom
        };

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

        // Calculate velocity with more randomness
        // Instead of always moving toward the center, create a more varied trajectory
        let center = Vec2::ZERO;
        let position = Vec2::new(x, y);

        // Base direction is toward the center, but add some randomness
        let base_direction = (center - position).normalize();

        // Add random angle deviation (up to 60 degrees)
        let angle_deviation = rand::random_range(-PI / 3.0..PI / 3.0);
        let rotation_matrix = Mat2::from_angle(angle_deviation);
        let direction = rotation_matrix.mul_vec2(base_direction);

        // Randomize speed
        let speed = rand::random_range(50.0..150.0);
        let linear_velocity = LinearVelocity(direction * speed);

        // Random angular velocity for rotation
        let angular_speed =
            if rand::random_bool(0.5) { -1.0 } else { 1.0 } * rand::random_range(1.0..TAU);
        let angular_velocity = AngularVelocity(angular_speed);

        // Spawn a single asteroid
        commands.spawn((
            Asteroid,
            RigidBody::Dynamic,
            Collider::circle(45.0),
            Sprite::from_atlas_image(
                space_assets.sheet.clone(),
                TextureAtlas {
                    layout: space_assets.texture_atlas_layout.clone(),
                    index: asteroid_indices[rand::random_range(0..asteroid_indices.len())],
                },
            ),
            Transform {
                translation: Vec3::new(x, y, 1.0),
                ..default()
            },
            linear_velocity,
            angular_velocity,
        ));
    }
}

fn check_asteroid_bounds(
    mut commands: Commands,
    mut asteroids: Query<(Entity, &Transform), With<Asteroid>>,
    window: Single<&Window>,
    camera: Single<&Transform, With<Camera2d>>,
) {
    // Use the same screen size calculation as in spawn_wave
    let visible_width = window.width() * camera.scale.x;
    let visible_height = window.height() * camera.scale.y;
    let screen_center_x = camera.translation.x;
    let screen_center_y = camera.translation.y;

    // Add a margin to ensure asteroids are fully off-screen before despawning
    let margin = 250.0;

    for (entity, transform) in asteroids.iter_mut() {
        let position = transform.translation;

        // Check if asteroid is outside the screen bounds with margin
        if position.x < screen_center_x + -visible_width / 2.0 - margin
            || position.x > screen_center_x + visible_width / 2.0 + margin
            || position.y < screen_center_y + -visible_height / 2.0 - margin
            || screen_center_y + position.y > visible_height / 2.0 + margin
        {
            // Despawn the asteroid
            commands.entity(entity).despawn();
        }
    }
}
