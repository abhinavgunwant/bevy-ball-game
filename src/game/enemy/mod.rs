use bevy::prelude::*;

pub mod components;
pub mod resources;
pub mod systems;

use crate::{ AppState, game::SimulationState };
use systems::*;
use resources::EnemySpawnTimer;

pub const NUMBER_OF_ENEMIES: usize = 4;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0;
pub const SPR_ENEMY: &str = "sprites/ball_red_large.png";

pub const AUD_ENEMY_COLL1: &str = "audio/pluck_001.ogg";
pub const AUD_ENEMY_COLL2: &str = "audio/pluck_002.ogg";

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            .add_systems(Update, (
                enemy_movement,
                tick_enemy_spawn_timer,
                spawn_enemies_over_time,
            )
                .run_if(in_state(AppState::Game))
                .run_if(in_state(SimulationState::Running))
        )
        .add_systems(OnExit(AppState::Game), despawn_enemies);
    }
}

