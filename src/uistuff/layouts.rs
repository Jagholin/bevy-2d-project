use crate::uistuff::config::*;
use crate::uistuff::utils::*;
use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};

#[derive(Clone, Default, Debug)]
pub struct NodeModifier {
    pub grid_column: Option<GridPlacement>,
    pub grid_row: Option<GridPlacement>,
}

impl NodeModifier {
    pub fn new() -> Self {
        NodeModifier {
            ..Default::default()
        }
    }
    pub fn set_grid_column(mut self, pl: GridPlacement) -> Self {
        self.grid_column = Some(pl);
        self
    }
    pub fn set_grid_row(mut self, pl: GridPlacement) -> Self {
        self.grid_row = Some(pl);
        self
    }
    pub fn modify(&self, n: Node) -> Node {
        let mut new_node = Node { ..n };
        if let Some(val) = self.grid_column {
            new_node.grid_column = val;
        }
        if let Some(val) = self.grid_row {
            new_node.grid_row = val;
        }
        new_node
    }
}

pub fn text_box(
    text: impl Into<String>,
    font: Handle<Font>,
    node_modifier: NodeModifier,
    style: BackgroundForeground,
) -> impl Bundle {
    (
        node_modifier.modify(Node {
            border: UiRect::all(Val::Px(2.0)),
            padding: UiRect::axes(Val::Auto, Val::Px(5.0)),
            ..Default::default()
        }),
        BackgroundColor(style.back_color),
        children![(
            Text::new(text),
            TextColor(style.fore_color),
            TextFont {
                font,
                font_size: 32.0,
                ..Default::default()
            }
        )],
    )
}

pub fn button_box(
    text: impl Into<String>,
    font: Handle<Font>,
    node_modifier: NodeModifier,
    style: ButtonStyle,
) -> impl Bundle {
    let result = text_box(text, font, node_modifier, style.normal_colors);
    (result, ChangeColorOnHover::from(style), Button)
}

pub fn grid_center_layout(
    mut command: Commands,
    extra_components: impl Bundle,
    func: impl FnOnce(&mut RelatedSpawnerCommands<ChildOf>),
) {
    command
        .spawn((
            Node {
                display: Display::Grid,
                min_height: Val::Vh(100.0),
                min_width: Val::Vw(100.0),
                grid_auto_rows: vec![GridTrack::min_content()],
                grid_template_columns: vec![
                    RepeatedGridTrack::fr(1, 1.0),
                    RepeatedGridTrack::auto(1),
                    RepeatedGridTrack::fr(1, 1.0),
                ],
                ..Default::default()
            },
            extra_components,
        ))
        .with_children(func);
}
