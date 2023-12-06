pub mod events;
mod systems;

mod enemy;
mod player;
mod score;
mod star;

use enemy::{ EnemyPlugin, resources::* };
use player::PlayerPlugin;
use score::{ ScorePlugin, resources::* };
use star::{ StarPlugin, resources::* };

use events::*;
use systems::*;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EnemyPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(ScorePlugin)
        .add_plugins(StarPlugin)
        .init_resource::<Score>()
        .init_resource::<HighScores>()
        .init_resource::<StarSpawnTimer>()
        .init_resource::<EnemySpawnTimer>()
        .add_event::<GameOver>()
        .add_systems(Startup, setup)
        .add_systems(Update, ( exit_game, handle_game_over ))
        .run();
}

