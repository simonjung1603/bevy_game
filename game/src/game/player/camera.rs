use bevy::prelude::*;

pub fn track_player(
    player: Single<&Transform, (With<super::component::Player>, Without<Camera>)>,
    mut camera: Single<&mut Transform, (With<Camera>, Without<super::component::Player>)>,
    time: Res<Time>,
) {
    const LERP_FACTOR: f32 = 2.0;

    let direction = player.translation.xy().extend(camera.translation.z);
    camera.translation = camera
        .translation
        .lerp(direction, time.delta_secs() * LERP_FACTOR);
}
