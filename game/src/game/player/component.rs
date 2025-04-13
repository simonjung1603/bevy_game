use avian2d::prelude::{
    AngularVelocity, Collider, ExternalForce, ExternalImpulse, LinearVelocity, RigidBody,
};
use bevy::prelude::*;

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    pub sprite: Sprite,
    pub player: Player,
    pub linear_velocity: LinearVelocity,
    pub angular_velocity: AngularVelocity,
    pub rigid_body: RigidBody,
    pub collider: Collider,
}

#[derive(Component, Default)]
#[require(ExternalForce, ExternalImpulse)]
pub struct Player;
