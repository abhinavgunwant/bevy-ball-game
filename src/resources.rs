use bevy::prelude::{ Resource, Timer, TimerMode };

pub const ENEMY_SPAWN_TIME: f32 = 5.0;
pub const STAR_SPAWN_TIME: f32 = 1.0;

#[derive(Resource, Default)]
pub struct Score {
    pub value: u32,
}

#[derive(Resource, Default, Debug)]
pub struct HighScores {
    pub scores: Vec<(String, u32)>,
}

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> EnemySpawnTimer  {
        EnemySpawnTimer { timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating) }
    }
}

#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer,
}

impl Default for StarSpawnTimer {
    fn default() -> StarSpawnTimer  {
        StarSpawnTimer { timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating) }
    }
}

