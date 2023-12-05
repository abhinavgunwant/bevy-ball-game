pub mod components;
pub mod events;
pub mod resources;
mod systems;

use events::*;
use resources::*;
use systems::*;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Score>()
        .init_resource::<HighScores>()
        .init_resource::<StarSpawnTimer>()
        .init_resource::<EnemySpawnTimer>()
        .add_event::<GameOver>()
        .add_systems(Startup, setup)
        .add_systems(Update, (
            player_movement, enemy_movement, enemy_player_collision,
            player_star_collision, update_score, tick_star_spawn_timer,
            spawn_stars_over_time, tick_enemy_spawn_timer,
            spawn_enemies_over_time, exit_game, handle_game_over,
            update_high_scores, high_scores_updated
        ))
        .run();
}

