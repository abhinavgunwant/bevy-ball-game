pub mod events;
mod systems;

mod game;
mod main_menu;

use game::{ GamePlugin, ui::GameUIPlugin };
use main_menu::MainMenuPlugin;
use systems::*;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_plugins(GamePlugin)
        .add_plugins(GameUIPlugin)
        .add_plugins(MainMenuPlugin)
        .add_systems(Startup, startup)
        .add_systems(OnEnter(AppState::Game), setup)
        .add_systems(Update, (
            exit_game, handle_game_over, transition_to_game_state,
            transition_to_main_menu_state,
        ))
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}

