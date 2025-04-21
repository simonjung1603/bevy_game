use avian2d::prelude::CollidingEntities;
use bevy::prelude::*;

use super::{Experience, PickupRadius, Xp};

pub fn pickup_xp(
    mut cmds: Commands,
    pickups: Query<&CollidingEntities, With<PickupRadius>>,
    xp_droplets: Query<(Entity, &Xp)>,
    mut xp: ResMut<Experience>,
) {
    for colliding_entitirs in &pickups {
        for (xp_entity, Xp(xp_value)) in colliding_entitirs
            .0
            .iter()
            .filter_map(|collided| xp_droplets.get(*collided).ok())
        {
            xp.current += *xp_value;
            cmds.entity(xp_entity).despawn_recursive();
        }
    }
}

pub fn level_up(mut xp: ResMut<Experience>) {
    if xp.current >= xp.target {
        xp.current %= xp.target;
        xp.target *= 2;
    }
}
