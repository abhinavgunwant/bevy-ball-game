use crate::{
    game::{
        SimulationState,
        enemy::{ *, components::Enemy },
        player::{ systems::{ SPR_PLAYER, PLAYER_SIZE }, components::Player },
        star::{ *, components::Star },
    },
    AppState, events::GameOver
};

use bevy::{ prelude::*, window::PrimaryWindow, app::AppExit };
use rand::prelude::*;

pub fn startup(
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

pub fn transition_to_game_state(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::G) {
        match app_state.get() {
            AppState::Game => {}

            _ => {
                commands.insert_resource(NextState(Some(AppState::Game)));
                println!("Entered AppState::Game");
            }
        }
    }
}

pub fn transition_to_main_menu_state(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::M) {
        match app_state.get() {
            AppState::MainMenu => {}

            _ => {
                commands.insert_resource(NextState(Some(AppState::MainMenu)));
                commands.insert_resource(NextState(Some(SimulationState::Paused)));
                println!("Entered AppState::MainMenu");
            }
        }
    }
}

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

pub fn handle_game_over(
    mut commands: Commands,
    mut game_over_event_reader: EventReader<GameOver>
) {
    for event in game_over_event_reader.read() {
        println!("Game Over! Your final score is: {}", event.score.to_string());
        commands.insert_resource(NextState(Some(AppState::GameOver)));
    }
}

