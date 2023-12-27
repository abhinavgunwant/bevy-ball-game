use bevy::prelude::*;

use crate::{
    game::ui::{ styles::*, hud::components::* },
    main_menu::styles::*,
};

pub fn spawn_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_hud(&mut commands, &asset_server);
}

pub fn despawn_hud() {}

fn build_hud(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) -> Entity {
    let mut style = GAME_MENU_STYLE.clone();

    style.flex_direction = FlexDirection::Row;
    style.position_type = PositionType::Absolute;
    style.justify_content = JustifyContent::SpaceBetween;
    style.align_items = AlignItems::Center;
    style.top = Val::Px(16.0);
    style.width = Val::Percent(100.0);
    style.padding = UiRect::new(
        Val::Px(0.0), Val::Px(16.0), Val::Px(0.0), Val::Px(16.0)
    );

    let hud_entity = commands.spawn(NodeBundle {
        style: style.clone(),
        background_color: COLOR_TRANSPARENT.into(),
        ..default()
    }).with_children(|parent| {
        let mut score_style = style.clone();

        score_style.justify_content = JustifyContent::FlexStart;
        score_style.width = Val::Auto;
        score_style.padding = UiRect::new(
            Val::Px(16.0), Val::Px(16.0), Val::Px(16.0), Val::Px(16.0)
        );
        score_style.top = Val::Px(0.0);
        score_style.position_type = PositionType::Relative;

        // Score
        parent.spawn(NodeBundle {
            style: score_style.clone(),
            background_color: COLOR_GREY.into(),
            ..default()
        }).with_children(|parent| {
            let mut image_style = IMAGE_STYLE.clone();

            image_style.width = Val::Px(32.0);
            image_style.height = Val::Px(32.0);

            let mut text_style = get_title_text_style(&asset_server);
            text_style.font_size = 32.0;

            // Star Image
            parent.spawn(ImageBundle {
                style: image_style,
                image: asset_server.load("sprites/star.png").into(),
                ..default()
            });
            
            // Score Text
            parent.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![TextSection::new("0", text_style)],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    ..default()
                },
                ScoreText {},
            ));
        });

        // Enemies
        parent.spawn(NodeBundle {
            style: score_style,
            background_color: COLOR_GREY.into(),
            ..default()
        }).with_children(|parent| {
            let mut image_style = IMAGE_STYLE.clone();

            image_style.width = Val::Px(32.0);
            image_style.height = Val::Px(32.0);

            let mut text_style = get_title_text_style(&asset_server);
            text_style.font_size = 32.0;

            // Enemy Image
            parent.spawn(ImageBundle {
                style: image_style,
                image: asset_server.load("sprites/ball_red_large.png").into(),
                ..default()
            });
            
            // Enemies Text
            parent.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![TextSection::new("0", text_style)],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    ..default()
                },
                EnemyText {},
            ));
        });
    }).id();

    hud_entity
}

pub fn spawn_fps_counter(mut commands: Commands) {
    let mut fps_root_style = GAME_MENU_STYLE.clone();
    fps_root_style.position_type = PositionType::Absolute;
    fps_root_style.left = Val::Px(0.0);
    fps_root_style.top = Val::Px(0.0);

    commands.spawn((
        NodeBundle {
            background_color: COLOR_TRANSPARENT.into(),
            z_index: ZIndex::Global(i32::MAX),
            style: fps_root_style.into(),
            ..default()
        },
        FpsRoot {},
    )).with_children(|parent| {
        let style = TextStyle {
            font_size: 16.0,
            color: Color::WHITE,
            ..default()
        };

        parent.spawn((
            TextBundle {
                text: Text::from_sections([
                    TextSection {
                        value: "FPS: ".into(),
                        style: style.clone(),
                    },
                    TextSection {
                        value: "N/A".into(),
                        style,
                    }
                ]),
                ..default()
            },
            FpsText {},
        ));
    });
}

