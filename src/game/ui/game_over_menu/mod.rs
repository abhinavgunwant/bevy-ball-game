pub mod systems;
pub mod components;
pub mod styles;

use bevy::prelude::*;

use crate::{
    AppState,
    game::ui::game_over_menu::systems::{
        interactions::*, layout::*, updates::*,
    },
};

pub struct GameOverMenuPlugin;

impl Plugin for GameOverMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::GameOver), spawn_game_over_menu)
            .add_systems(Update, (
                interact_with_restart_button,
                interact_with_main_menu_button,
                interact_with_quit_button,
                update_final_score_text,
            ).run_if(in_state(AppState::GameOver)))
            .add_systems(OnExit(AppState::GameOver), despawn_game_over_menu);
    }
}

