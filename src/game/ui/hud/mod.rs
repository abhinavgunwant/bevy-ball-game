mod systems;
mod components;

pub struct HudPlugin;

use bevy::prelude::*;
use crate::{ AppState, game::ui::hud::systems::{ updates::*, layout::* }};

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Game), spawn_hud)
            .add_systems(Update, (
                    update_score_text, update_enemy_text
                ).run_if(in_state(AppState::Game))
            )
            .add_systems(OnExit(AppState::Game), despawn_hud);
    }
}

