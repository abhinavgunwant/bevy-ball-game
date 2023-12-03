use bevy::prelude::*;
use bevy::{ audio::PlaybackMode, window::PrimaryWindow };
use rand::prelude::*;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0;
pub const NUMBER_OF_ENEMIES: usize = 4;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0;
pub const NUMBER_OF_STARS: usize = 10;
pub const STAR_SIZE: f32 = 30.0;
pub const STAR_SPAWN_TIME: f32 = 1.0;

// Sprites
pub const SPR_PLAYER: &str = "sprites/ball_blue_large.png";
pub const SPR_ENEMY: &str = "sprites/ball_red_large.png";
pub const SPR_STAR: &str = "sprites/star.png";

// Audio
pub const AUD_ENEMY_COLL1: &str = "audio/pluck_001.ogg";
pub const AUD_ENEMY_COLL2: &str = "audio/pluck_002.ogg";
pub const AUD_PLAYER_DEAD: &str = "audio/explosionCrunch_000.ogg";
pub const AUD_STAR_COLLECT: &str = "audio/laserLarge_000.ogg";

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Score>()
        .init_resource::<StarSpawnTimer>()
        .add_systems(Startup, setup)
        .add_systems(Update, (
            player_movement, enemy_movement, enemy_player_collision,
            player_star_collision, update_score, tick_star_spawn_timer,
            spawn_stars_over_time,
        ))
        .run();
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
}

#[derive(Component)]
pub struct Star {}

#[derive(Resource, Default)]
pub struct Score {
    pub value: u32,
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

pub fn setup(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let window_width = window.width();
    let window_height = window.height();

    println!("Window width: {}, height: {}", window_width, window_height);
    let posx = window_width / 2.0;
    let posy = window_height / 2.0;

    // spawn camera
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(posx, posy, 999.0), ..default()
    });

    // spawn player
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(SPR_PLAYER),
            transform: Transform::from_xyz(posx, posy, 1.0),
            ..default()
        },
        Player,
    ));

    let half_player_size = PLAYER_SIZE / 2.0;

    // spawn enemies
    for _ in 0..NUMBER_OF_ENEMIES {
        let random_x = half_player_size + (random::<f32>() * (window_width - PLAYER_SIZE));
        let random_y = half_player_size + (random::<f32>() * (window_height - PLAYER_SIZE));

        commands.spawn((
            SpriteBundle {
                texture: asset_server.load(SPR_ENEMY),
                transform: Transform::from_xyz(random_x, random_y, 1.0),
                ..default()
            },
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            },
        ));
    }

    // Spawn stars
    for _ in 0..NUMBER_OF_STARS {
        let random_x = random::<f32>() * window_width;
        let random_y = random::<f32>() * window_height;

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load(SPR_STAR),
                ..default()
            },
            Star {},
        ));
    }
}

pub fn player_movement (
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;
        let window = window_query.get_single().unwrap();

        let half_player_size = PLAYER_SIZE / 2.0;
        let x_min = 0.0 + half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = 0.0 + half_player_size;
        let y_max = window.height() - half_player_size;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        let mut tr = transform.translation + (direction * PLAYER_SPEED * time.delta_seconds());

        if tr.x < x_min {
            tr.x = x_min;
        } else if tr.x > x_max {
            tr.x = x_max;
        }

        if tr.y < y_min {
            tr.y = y_min;
        } else if tr.y > y_max {
            tr.y = y_max;
        }

        transform.translation = tr;
    }
}

pub fn enemy_movement(
    mut commands: Commands,
    mut enemy_query: Query<(&mut Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let half_enemy_size = ENEMY_SIZE / 2.0;
    let x_min = 0.0 + half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = 0.0 + half_enemy_size;
    let y_max = window.height() - half_enemy_size;

    for (mut transform, mut enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation +=
            direction * ENEMY_SPEED * time.delta_seconds();

        let mut tr = transform.translation;
        let mut direction_changed: bool = false;

        if tr.x < x_min {
            tr.x = x_min;
            enemy.direction.x *= -1.0;
            direction_changed = true;
        } else if tr.x > x_max {
            tr.x = x_max;
            enemy.direction.x *= -1.0;
            direction_changed = true;
        }

        if tr.y < y_min {
            tr.y = y_min;
            enemy.direction.y *= -1.0;
            direction_changed = true;
        } else if tr.y > y_max {
            tr.y = y_max;
            enemy.direction.y *= -1.0;
            direction_changed = true;
        }

        transform.translation = tr;

        if direction_changed {
            commands.spawn((
                AudioBundle {
                    source: if random::<f32>() > 0.5 {
                        asset_server.load(AUD_ENEMY_COLL1)
                    } else {
                        asset_server.load(AUD_ENEMY_COLL2)
                    },
    
                    settings: PlaybackSettings {
                        mode: PlaybackMode::Despawn,
                        ..default()
                    }
                },
            ));
        }
    }
}

pub fn enemy_player_collision(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);

            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE / 2.0;

            if distance < player_radius + enemy_radius {
                println!("Game Over!");

                commands.spawn((
                    AudioBundle {
                        source: asset_server.load(AUD_PLAYER_DEAD),
                        settings: PlaybackSettings {
                            mode: PlaybackMode::Despawn,
                            ..default()
                        }
                    },
                ));

                commands.entity(player_entity).despawn();
            }
        }
    }
}

pub fn player_star_collision(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (star_entity, star_transform) in star_query.iter() {
            let distance = player_transform
                .translation
                .distance(star_transform.translation);

            let player_radius = PLAYER_SIZE / 2.0;
            let star_radius = STAR_SIZE / 2.0;

            if distance < player_radius + star_radius {
                commands.spawn((
                    AudioBundle {
                        source: asset_server.load(AUD_STAR_COLLECT),
                        settings: PlaybackSettings {
                            mode: PlaybackMode::Despawn,
                            ..default()
                        }
                    },
                ));

                commands.entity(star_entity).despawn();
                score.value += 1;
            }
        }
    }
}

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value.to_string());
    }
}

pub fn tick_star_spawn_timer(
    mut star_spawn_timer: ResMut<StarSpawnTimer>,
    time: Res<Time>
) {
    star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<StarSpawnTimer>,
) {
    if star_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load(SPR_STAR),
                ..default()
            },
            Star {},
        ));
    }
}

