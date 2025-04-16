use avian2d::prelude::LinearVelocity;
pub use bevy::prelude::*;
use bevy_enoki::prelude::*;

use crate::GameState;

pub fn plugin(app: &mut App) {
    app.add_plugins(EnokiPlugin).add_systems(
        FixedUpdate,
        adjust_based_on_parent.run_if(in_state(GameState::Game)),
    );
}

fn adjust_based_on_parent(
    parents: Query<&LinearVelocity>,
    mut thrusters: Query<(&Parent, &mut ParticleEffectInstance)>,
) {
    for (parent, mut thruster) in &mut thrusters {
        let velocity = parents.get(parent.get()).unwrap();
        if let Some(thruster) = thruster.0.as_mut() {
            let assumed_max_vel = velocity.length_squared() / 300000.0;
            let Rval(val, _) = thruster.linear_speed.as_mut().unwrap();
            *val = 0.0.lerp(400.0, assumed_max_vel);

            thruster.spawn_amount = 1 + ((20 - 1) as f32 * assumed_max_vel) as u32;
            thruster.spawn_rate = 0.0;
            thruster.lifetime.0 = 0.2;
        }
    }
}
