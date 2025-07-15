use bevy::{
    color::palettes::{css::*, tailwind::*},
    prelude::*,
};
#[derive(Clone, Copy, Debug)]
pub struct BackgroundForeground {
    pub back_color: Color,
    pub fore_color: Color,
}

#[derive(Clone, Copy, Debug)]
pub struct ButtonStyle {
    pub normal_colors: BackgroundForeground,
    pub hover_colors: BackgroundForeground,
}

#[derive(Resource, Clone, Copy, Debug)]
pub struct UiStyle {
    pub button_style: ButtonStyle,
    pub back_color: Color,
}

pub const STANDARD_STYLE: UiStyle = UiStyle {
    back_color: Color::Srgba(VIOLET_100),
    button_style: ButtonStyle {
        normal_colors: BackgroundForeground {
            back_color: Color::Srgba(PINK_400),
            fore_color: Color::Srgba(WHITE),
        },
        hover_colors: BackgroundForeground {
            back_color: Color::Srgba(PINK_200),
            fore_color: Color::Srgba(GRAY_700),
        },
    },
};
