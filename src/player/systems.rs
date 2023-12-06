use crate::{
    enemy::{ ENEMY_SIZE, components::Enemy },
    player::components::Player,
    star::{ *, components::Star },
    score::resources::Score, events::GameOver,
};

use bevy::{ prelude::*, window::PrimaryWindow, audio::PlaybackMode };

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0;
pub const AUD_PLAYER_DEAD: &str = "audio/explosionCrunch_000.ogg";
pub const SPR_PLAYER: &str = "sprites/ball_blue_large.png";

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

pub fn enemy_player_collision(
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<GameOver>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);

            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE / 2.0;

            if distance < player_radius + enemy_radius {
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

                game_over_event_writer.send(GameOver { score: score.value });
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

