pub mod enemy;
pub mod player;
mod score;
pub mod star;
mod systems;
pub mod ui;

use bevy::prelude::*;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use systems::*;
use crate::{ AppState, events::GameOver };

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SimulationState>()
            .add_event::<GameOver>()
            .add_plugins(EnemyPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(ScorePlugin)
            .add_plugins(StarPlugin)
            .add_systems(OnEnter(AppState::Game), pause_simulation)
            .add_systems(Update, toggle_simulation.run_if(
                in_state(AppState::Game)
            ));
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}

