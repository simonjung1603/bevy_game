use avian2d::prelude::{Collider, CollidingEntities, Sensor};
use bevy::prelude::*;

pub mod pickup;

/// Resource tracking the experience player stat
#[derive(Resource)]
pub struct Experience {
    pub current: usize,
    pub target: usize,
}

/// Component allowing loot to grant experience when picked up
#[derive(Component, Deref, DerefMut)]
pub struct Xp(pub usize);

/// Component specifying a pickup radius
///
/// Needs to be paired with a [Collider], or use the [XpPickup] bundle instead
#[derive(Component, Deref, DerefMut)]
#[require(Sensor, CollidingEntities)]
pub struct PickupRadius(pub f32);

/// Component granting an entity the ability to pickup experience in the specified radius
#[derive(Bundle)]
pub struct XpPickup {
    radius: PickupRadius,
    collider: Collider,
}

impl Default for XpPickup {
    fn default() -> Self {
        Self {
            radius: PickupRadius(150.0),
            collider: Collider::circle(150.0),
        }
    }
}
