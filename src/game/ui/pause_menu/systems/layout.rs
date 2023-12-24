use bevy::prelude::*;

use crate::{
    game::ui::{ styles::*, pause_menu::components::* },
    main_menu::styles::*,
};

pub fn spawn_pause_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let main_menu_entity = build_pause_menu(&mut commands, &asset_server);
}

pub fn despawn_pause_menu(
    mut commands: Commands,
    pause_menu_query: Query<Entity, With<PauseMenu>>
) {
    if let Ok(pause_menu_entity) = pause_menu_query.get_single() {
        commands.entity(pause_menu_entity).despawn_recursive();
    }
}

fn build_pause_menu(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) -> Entity {
    let pause_menu_entity = commands.spawn((
        NodeBundle {
            style: GAME_MENU_STYLE,
            background_color: Color::rgba(0.0, 0.0, 0.0, 0.2).into(),
            ..default()
        },
        PauseMenu,
    ))
    .with_children(|parent| {
        parent.spawn(TextBundle {
            text: Text {
                sections: vec![TextSection::new(
                    "Paused",
                    get_title_text_style(&asset_server),
                )],
                alignment: TextAlignment::Center,
                ..default()
            },
            ..default()
        });

        // Resume Button
        parent.spawn((
            ButtonBundle {
                style: BUTTON_STYLE,
                background_color: NORMAL_BUTTON_COLOR.into(),
                ..default()
            },
            ResumeButton,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Resume",
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
                        "Quit",
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

    pause_menu_entity
}

