use bevy::prelude::*;

use crate::main_menu::styles::MAIN_MENU_STYLE;

pub const GAME_MENU_STYLE: Style = {
    let mut style = MAIN_MENU_STYLE;

    style.padding = UiRect::new(
        Val::Px(64.0), Val::Px(64.0), Val::Px(64.0), Val::Px(64.0)
    );
    style.width = Val::Auto;
    style.height = Val::Auto;
    style.margin = UiRect::new(Val::Auto, Val::Auto, Val::Auto, Val::Auto);

    style
};

pub const COLOR_GREY: Color = Color::rgba(0.0, 0.0, 0.0, 0.2);
pub const COLOR_TRANSPARENT: Color = Color::rgba(0.0, 0.0, 0.0, 0.0);

