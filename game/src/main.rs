#![allow(clippy::type_complexity)]
mod game;
mod menu;
mod util;

use avian2d::{prelude::Gravity, PhysicsPlugins};
use bevy::prelude::*;
use bevy_kenney_assets::KenneyAssetPlugin;

#[cfg(debug_assertions)]
use {
    avian2d::prelude::PhysicsDebugPlugin, bevy::input::common_conditions::input_toggle_active,
    bevy_inspector_egui::quick::WorldInspectorPlugin,
};

const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

// Enum that will be used as a global state for the game
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Menu,
    Loading,
    Game,
}

fn main() {
    let mut app = App::new();

    // Base setup
    app.add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .enable_state_scoped_entities::<GameState>()
        .add_systems(Startup, setup);

    // Physics
    app.add_plugins(PhysicsPlugins::default().with_length_unit(100.0))
        .insert_resource(Gravity::ZERO);

    // Assets
    app.add_plugins(KenneyAssetPlugin);

    // Menu
    app.add_plugins(menu::menu_plugin);

    // Game
    app.add_plugins(game::game_plugin);

    // Debug helpers
    #[cfg(debug_assertions)]
    app.add_plugins((
        PhysicsDebugPlugin::default(),
        WorldInspectorPlugin::new().run_if(input_toggle_active(false, KeyCode::F2)),
    ));

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
