mod game;
mod menu;
mod util;

use avian2d::{
    prelude::{Gravity, PhysicsDebugPlugin},
    PhysicsPlugins,
};
use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_kenney_assets::KenneyAssetPlugin;

const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

// Enum that will be used as a global state for the game
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Menu,
    Loading,
    Game,
}

// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
enum DisplayQuality {
    Low,
    Medium,
    High,
}

// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
struct Volume(u32);

fn main() {
    let mut app = App::new();

    // Base setup
    app.add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_systems(Startup, setup);

    // Physics
    app.add_plugins(PhysicsPlugins::default().with_length_unit(100.0))
        .insert_resource(Gravity::ZERO);

    // Assets
    app.add_plugins(KenneyAssetPlugin);

    // Menu
    app.insert_resource(DisplayQuality::Medium)
        .insert_resource(Volume(7))
        .add_plugins(menu::menu_plugin);

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
