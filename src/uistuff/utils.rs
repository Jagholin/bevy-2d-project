use crate::uistuff::config::*;
use bevy::prelude::*;

#[derive(Component, Debug)]
#[require(Interaction)]
pub struct ChangeColorOnHover {
    pub normal_color: BackgroundForeground,
    pub hover_color: BackgroundForeground,
}

impl From<ButtonStyle> for ChangeColorOnHover {
    fn from(value: ButtonStyle) -> Self {
        Self {
            normal_color: value.normal_colors,
            hover_color: value.hover_colors,
        }
    }
}

pub fn change_color_on_hover(
    mut comms: Commands,
    backgr: Query<
        (
            &Children,
            &ChangeColorOnHover,
            &mut BackgroundColor,
            &Interaction,
        ),
        Changed<Interaction>,
    >,
) {
    for (children, colors, mut col, int) in backgr {
        let text_color = if matches!(int, Interaction::Hovered) {
            colors.hover_color.fore_color
        } else {
            colors.normal_color.fore_color
        };
        for child in children.iter() {
            comms
                .entity(child)
                .entry::<TextColor>()
                .and_modify(move |mut ent| {
                    //ent.0 = colors.hover_color.fore_color;
                    ent.0 = text_color;
                });
        }
        match int {
            Interaction::Hovered => col.0 = colors.hover_color.back_color,
            Interaction::None => col.0 = colors.normal_color.back_color,
            _ => (),
        }
    }
}

pub struct UiUtilsPlugin;

impl Plugin for UiUtilsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, change_color_on_hover);
    }
}
