use bevy::prelude::*;

#[derive(Component, Debug)]
#[require(Interaction)]
pub struct ChangeColorOnHover {
    pub normal_color: Color,
    pub hover_color: Color,
}

pub fn change_color_on_hover(
    backgr: Query<(&ChangeColorOnHover, &mut BackgroundColor, &Interaction), Changed<Interaction>>,
) {
    for (colors, mut col, int) in backgr {
        match int {
            Interaction::Hovered => col.0 = colors.hover_color,
            Interaction::None => col.0 = colors.normal_color,
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
