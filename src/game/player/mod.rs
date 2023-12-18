use bevy::prelude::*;

pub mod components;
pub mod systems;

use crate::{ AppState, game::SimulationState };
use systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            player_movement, enemy_player_collision, player_star_collision,
            )
            .run_if(in_state(AppState::Game))
            .run_if(in_state(SimulationState::Running))
        )
        .add_systems(OnExit(AppState::Game), despawn_player);
    }
}

