use bevy::{
    asset::AssetMetaCheck,
    prelude::{default, App, Camera2dBundle, ClearColor, Commands, PluginGroup, Startup, States},
    window::{PresentMode, Window, WindowPlugin, WindowResolution},
    DefaultPlugins,
};

use constants::{BACKGROUND_COLOR, WINDOW_HEIGHT, WINDOW_WIDTH};

mod board;
mod brick;
mod common_entity;
mod constants;
mod data;
mod game;
mod gameover;
mod menu;
mod menu_help;
mod position;
mod utils;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Menu,
    Game,
    GameOver,
    HelpMenu,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(AssetMetaCheck::Never)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "TETRIS".to_string(),
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                present_mode: PresentMode::AutoVsync,
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        // .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, camera_setup)
        .add_state::<GameState>()
        .add_plugins(menu::MenuPlugin)
        .add_plugins(game::GamePlugin)
        .add_plugins(gameover::GameOverPlugin)
        .add_plugins(menu_help::MenuHelpPlugin)
        .run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
