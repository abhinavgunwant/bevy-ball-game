use bevy::prelude::*;

pub mod components;
pub mod resources;
pub mod systems;

pub const NUMBER_OF_STARS: usize = 10;
pub const STAR_SIZE: f32 = 30.0;
pub const AUD_STAR_COLLECT: &str = "audio/laserLarge_000.ogg";
pub const SPR_STAR: &str = "sprites/star.png";

use resources::*;
use systems::*;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
            .add_systems(Update, (
                tick_star_spawn_timer, spawn_stars_over_time,
            ));
    }
}
