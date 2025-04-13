use super::component::Player;
use avian2d::prelude::{
    AngularVelocity, ExternalAngularImpulse, ExternalForce, ExternalImpulse, LinearVelocity,
};
use bevy::{math::FloatPow, prelude::*, text::cosmic_text::ttf_parser::loca};

pub fn print_position(
    player: Option<
        Single<(&Transform, &LinearVelocity, &AngularVelocity), (Changed<Transform>, With<Player>)>,
    >,
) {
    if let Some(player) = player {
        let (player, velocity, angular_velocity) = player.into_inner();
        //info!("{}: {:?} | {:?}", player.translation, velocity, angular_velocity);
    }
}

pub fn up(
    player: Single<
        (
            &Transform,
            &mut ExternalImpulse,
            &mut ExternalForce,
            &LinearVelocity,
            &AngularVelocity,
        ),
        With<Player>,
    >,
    key: Res<ButtonInput<KeyCode>>,
) {
    let (transform, mut impulse, mut force, lin_vel, ang_vel) = player.into_inner();

    const THRUST_IMPULSE: f32 = 300.0;
    const MAX_LINEAR_IMPULSE: f32 = THRUST_IMPULSE * THRUST_IMPULSE;
    const THRUST_FORCE: f32 = THRUST_IMPULSE * 2.0;

    const ANGULAR_IMPULSE: f32 = 20.0;
    const MAX_ANGULAR_IMPULSE: f32 = 1.0;
    const ANGULAR_FORCE: f32 = 120.0;
    const ROTATION_OFFSET: f32 = 100.0;

    let impulse_magnitude = lin_vel.length_squared();
    let angular_impulse_magnitude = ang_vel.abs();

    if impulse_magnitude < MAX_LINEAR_IMPULSE && key.just_pressed(KeyCode::ArrowUp) {
        impulse.apply_impulse((transform.rotation * Vec3::Y * THRUST_IMPULSE).xy());
    }
    if key.pressed(KeyCode::ArrowUp) {
        force.apply_force((transform.rotation * Vec3::Y * THRUST_FORCE).xy());
    }

    let left_just_pressed = key.just_pressed(KeyCode::ArrowLeft);
    let left_pressed = left_just_pressed || key.pressed(KeyCode::ArrowLeft);
    let right_just_pressed = key.just_pressed(KeyCode::ArrowRight);
    let right_pressed = right_just_pressed || key.pressed(KeyCode::ArrowRight);

    info!("Linear impulse: {}", impulse_magnitude);
    info!("Angular impulse: {}", angular_impulse_magnitude);

    if angular_impulse_magnitude < MAX_ANGULAR_IMPULSE && (left_just_pressed || right_just_pressed)
    {
        let local_impulse = (transform.rotation * Vec3::Y * ANGULAR_IMPULSE).xy();
        let center = transform.translation.xy();
        let left = (transform.translation + transform.left() * ROTATION_OFFSET).xy();
        let right = (transform.translation + transform.right() * ROTATION_OFFSET).xy();
        impulse.apply_impulse_at_point(
            right_just_pressed as i32 as f32 * local_impulse
                - left_just_pressed as i32 as f32 * local_impulse,
            left,
            center,
        );
        impulse.apply_impulse_at_point(
            left_just_pressed as i32 as f32 * local_impulse
                - right_just_pressed as i32 as f32 * local_impulse,
            right,
            center,
        );
    }
    if left_pressed || right_pressed {
        let local_force = (transform.rotation * Vec3::Y * ANGULAR_FORCE).xy();
        let center = transform.translation.xy();
        let left = (transform.translation + transform.left() * ROTATION_OFFSET).xy();
        let right = (transform.translation + transform.right() * ROTATION_OFFSET).xy();
        force.apply_force_at_point(
            right_pressed as i32 as f32 * local_force - left_pressed as i32 as f32 * local_force,
            left,
            center,
        );
        force.apply_force_at_point(
            left_pressed as i32 as f32 * local_force - right_pressed as i32 as f32 * local_force,
            right,
            center,
        );
    }
}
