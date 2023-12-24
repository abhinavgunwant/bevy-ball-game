use bevy::{ prelude::*, app::AppExit };

use crate::{
    AppState,
    game::{ ui::pause_menu::components::*, SimulationState },
    main_menu::styles::{
        HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR, PRESSED_BUTTON_COLOR,
    },
};

pub fn interact_with_resume_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ResumeButton>)
    >,
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                simulation_state_next_state.set(SimulationState::Running);
            }

            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            }

            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}

pub fn interact_with_main_menu_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<MainMenuButton>)
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                app_state_next_state.set(AppState::MainMenu);
                simulation_state_next_state.set(SimulationState::Running);
            }

            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            }

            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}

pub fn interact_with_quit_button(
    mut app_exit_event_writer: EventWriter<AppExit>,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitButton>)
    >,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                app_exit_event_writer.send(AppExit);
            }

            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            }

            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}

