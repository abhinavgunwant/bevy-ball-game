use bevy::prelude::*;

pub mod components;
pub mod systems;

use systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            player_movement, enemy_player_collision, player_star_collision,
        ));
    }
}

