use bevy::{ prelude::*, window::PrimaryWindow, audio::PlaybackMode };
use rand::prelude::*;

use crate::game::enemy::{ *, components::Enemy, resources::EnemySpawnTimer };

pub fn despawn_enemies(
    mut commands: Commands,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    for enemy_entity in enemy_query.iter() {
        commands.entity(enemy_entity).despawn();
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

pub fn tick_enemy_spawn_timer(
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
    time: Res<Time>
) {
    enemy_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_enemies_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_spawn_timer: Res<EnemySpawnTimer>,
) {
    if enemy_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

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
}

