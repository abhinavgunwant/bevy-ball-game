mod systems;
mod components;

use bevy::prelude::*;

use crate::game::{
    SimulationState,
    ui::pause_menu::systems::{ interactions::*, layout::* }
};

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(SimulationState::Paused), spawn_pause_menu)
            .add_systems(Update, (
                interact_with_resume_button,
                interact_with_main_menu_button,
                interact_with_quit_button,
            ))
            .add_systems(OnExit(SimulationState::Paused), despawn_pause_menu);
    }
}

