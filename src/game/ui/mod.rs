pub mod game_over_menu;
pub mod hud;
pub mod pause_menu;
mod styles;

use bevy::prelude::*;

use game_over_menu::GameOverMenuPlugin;
use hud::HudPlugin;
use pause_menu::PauseMenuPlugin;

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(HudPlugin)
            .add_plugins(PauseMenuPlugin)
            .add_plugins(GameOverMenuPlugin);
    }
}

