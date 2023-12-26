use bevy::prelude::*;

use crate::{
    game::ui::{ styles::*, game_over_menu::components::* },
    main_menu::styles::*,
};

pub fn spawn_game_over_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    build_game_over_menu(&mut commands, &asset_server);
}

pub fn despawn_game_over_menu(
    mut commands: Commands,
    game_over_menu_query: Query<Entity, With<GameOverMenu>>
) {
    if let Ok(game_over_menu_entity) = game_over_menu_query.get_single() {
        commands.entity(game_over_menu_entity).despawn_recursive();
    }
}

fn build_game_over_menu(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) -> Entity {
    let game_over_menu_entity = commands.spawn((
        NodeBundle {
            style: GAME_MENU_STYLE,
            background_color: Color::rgba(0.0, 0.0, 0.0, 0.2).into(),
            ..default()
        },
        GameOverMenu,
    ))
    .with_children(|parent| {
        // "Game Over" menu title
        parent.spawn(TextBundle {
            text: Text {
                sections: vec![TextSection::new(
                    "Game Over",
                    get_title_text_style(&asset_server),
                )],
                alignment: TextAlignment::Center,
                ..default()
            },
            ..default()
        });

        // Final Score
        parent.spawn((
            TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Final Score: ",
                        get_title_text_style(&asset_server),
                    )],
                    alignment: TextAlignment::Center,
                    ..default()
                },
                ..default()
            },
            FinalScoreText {}
        ));

        // Resume Button
        parent.spawn((
            ButtonBundle {
                style: BUTTON_STYLE,
                background_color: NORMAL_BUTTON_COLOR.into(),
                ..default()
            },
            RestartButton,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Restart [G]",
                        get_button_text_style(asset_server),
                    )],
                    alignment: TextAlignment::Center,
                    ..default()
                },
                ..default()
            });
        });

        // Main Menu Button
        parent.spawn((
            ButtonBundle {
                style: BUTTON_STYLE,
                background_color: NORMAL_BUTTON_COLOR.into(),
                ..default()
            },
            MainMenuButton,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Main Menu",
                        get_button_text_style(asset_server),
                    )],
                    alignment: TextAlignment::Center,
                    ..default()
                },
                ..default()
            });
        });

        // Quit Button
        parent.spawn((
            ButtonBundle {
                style: BUTTON_STYLE,
                background_color: NORMAL_BUTTON_COLOR.into(),
                ..default()
            },
            QuitButton,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Quit [Esc]",
                        get_button_text_style(asset_server),
                    )],
                    alignment: TextAlignment::Center,
                    ..default()
                },
                ..default()
            });
        });
    })
    .id();

    game_over_menu_entity
}

