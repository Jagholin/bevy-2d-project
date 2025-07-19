use crate::uistuff::config::*;
use crate::uistuff::utils::*;
use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};

#[derive(Clone, Default, Debug)]
pub struct NodeModifier {
    pub grid_column: Option<GridPlacement>,
    pub grid_row: Option<GridPlacement>,
    pub is_root: bool,
    pub force_absolute_position: bool,
    pub is_hidden: bool,
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
    pub fn root() -> Self {
        NodeModifier {
            is_root: true,
            ..Default::default()
        }
    }
    pub fn force_absolute_pos(mut self) -> Self {
        self.force_absolute_position = true;
        self
    }
    pub fn spawn_hidden(mut self) -> Self {
        self.is_hidden = true;
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
        if self.is_root {
            new_node.min_width = Val::Vw(100.0);
            new_node.min_height = Val::Vh(100.0);
        }
        if self.force_absolute_position {
            new_node.position_type = PositionType::Absolute;
        }
        if self.is_hidden {
            new_node.display = Display::None;
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

pub fn vertically_centered(
    mut command: impl GenericSpawner,
    extra_components: impl Bundle,
    node_modifier: NodeModifier,
    func: impl FnOnce(&mut RelatedSpawnerCommands<ChildOf>),
) -> Entity {
    command
        .generic_spawn((
            extra_components,
            node_modifier.modify(Node {
                display: Display::Flex,
                align_items: AlignItems::Center,
                // justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Row,
                ..Default::default()
            }),
        ))
        .with_children(func)
        .id()
}

pub fn grid_hor_center_layout(
    mut command: impl GenericSpawner,
    extra_components: impl Bundle,
    node_modifier: NodeModifier,
    central_columns: u16,
    child_spawner: impl FnOnce(&mut RelatedSpawnerCommands<ChildOf>),
) -> Entity {
    command
        .generic_spawn((
            node_modifier.modify(Node {
                display: Display::Grid,
                min_width: Val::Percent(100.0),
                grid_auto_rows: vec![GridTrack::min_content()],
                grid_template_columns: vec![
                    RepeatedGridTrack::fr(1, 1.0),
                    RepeatedGridTrack::auto(central_columns),
                    RepeatedGridTrack::fr(1, 1.0),
                ],
                ..Default::default()
            }),
            extra_components,
        ))
        .with_children(child_spawner)
        .id()
}
