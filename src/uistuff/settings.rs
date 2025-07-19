use crate::uistuff::utils::*;
use bevy::{color::palettes::css::*, ecs::relationship::DescendantIter, prelude::*};

use super::{
    config::{BackgroundForeground, UiStyle},
    layouts::{NodeModifier, grid_hor_center_layout, text_box, vertically_centered},
};

#[derive(Resource, Clone, Debug, Deref, DerefMut)]
struct OpenedPopup(Option<Entity>);

#[derive(Component, Clone, Debug, Deref)]
struct UISelection(String);

#[derive(Component)]
struct DropdownMenuMenu;

#[derive(Component)]
struct DropdownMenuButton;

#[derive(Clone, Debug, PartialEq)]
struct LayoutInfo {
    position: Vec2,
    size: Vec2,
}

#[derive(Component, Default, Debug, Deref, DerefMut)]
struct PreviousLayout(Option<LayoutInfo>);

pub struct SettingsPlugin;

#[derive(Clone, Default, Debug)]
pub struct DropdownMenuItem {
    pub label: String,
}

#[derive(Clone, Default, Debug)]
pub struct DropdownMenu {
    pub items: Vec<DropdownMenuItem>,
}

// menu items
pub fn menu_item(label: DropdownMenuItem) -> impl Bundle {
    (
        Node {
            // width: Val::Percent(100.0),
            padding: UiRect::vertical(Val::Px(5.0)),
            ..Default::default()
        },
        ChangeColorOnHover {
            normal_color: BackgroundForeground {
                back_color: BLUE_VIOLET.into(),
                fore_color: WHITE.into(),
            },
            hover_color: BackgroundForeground {
                back_color: AQUAMARINE.into(),
                fore_color: BLACK.into(),
            },
        },
        BackgroundColor(BLUE_VIOLET.into()),
        children![(Text::new(label.label),)],
    )
}

pub fn dropdown_menu(
    mut commands: impl GenericSpawner,
    extra_components: impl Bundle,
    node_modifier: NodeModifier,
    menu: DropdownMenu,
) {
    commands
        .generic_spawn((
            extra_components,
            node_modifier.modify(Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Stretch,
                ..Default::default()
            }),
            BackgroundColor(BLUE.into()),
        ))
        .with_children(|parent| {
            for item in menu.items.iter() {
                parent.spawn(menu_item(item.clone()));
            }
        });
}

pub fn spawn_combobox(
    mut commands: impl GenericSpawner,
    menu: DropdownMenu,
    node_modifier: NodeModifier,
    ui_assets: Res<UIAssets>,
    ui_style: Res<UiStyle>,
) {
    let first_item = menu.items[0].label.clone();
    commands
        .generic_spawn((
            text_box(
                first_item.clone(),
                ui_assets
                    .font
                    .clone()
                    .expect("Font should be loaded already"),
                node_modifier,
                ui_style.button_style.normal_colors,
            ),
            DropdownMenuButton,
            UISelection(first_item),
            Name::new("Combobox"),
            PreviousLayout(None),
        ))
        .with_children(|parent| {
            dropdown_menu(
                parent,
                (DropdownMenuMenu, Name::new("Dropdown"), GlobalZIndex(100)),
                NodeModifier::new().force_absolute_pos().spawn_hidden(),
                menu,
            );
        })
        .observe(on_combobutton_clicked);
}

fn on_combobutton_clicked(
    mut trigger: Trigger<Pointer<Click>>,
    mut child_query: Query<&mut Node, With<DropdownMenuMenu>>,
    children_query: Query<&Children, With<DropdownMenuButton>>,
    mut opened_popup: ResMut<OpenedPopup>,
) {
    let target = trigger.target();
    for child in DescendantIter::new(&children_query, target) {
        if let Ok(mut my_node) = child_query.get_mut(child) {
            my_node.display = Display::Flex;
            opened_popup.0 = Some(child);
            trigger.propagate(false);
        }
    }
}

//creates combobox with an associated menu
#[allow(clippy::type_complexity)]
fn on_combobutton_layout(
    query: Query<
        (&ComputedNode, &Transform, &mut PreviousLayout, &Children),
        (With<DropdownMenuButton>, Changed<ComputedNode>),
    >,
    mut drop_query: Query<&mut Node, With<DropdownMenuMenu>>,
) {
    for (cn, tr, mut pl, children) in query {
        let plane_translation = tr.translation.xy();
        if let Some(ref li) = pl.0 {
            if li.position == plane_translation && li.size == cn.size {
                continue;
            }
        }
        info!("computed node location changed");
        let li = LayoutInfo {
            position: plane_translation,
            size: cn.size(),
        };
        for child in children {
            if let Ok(mut ch) = drop_query.get_mut(*child) {
                ch.top = Val::Px(li.size.y);
                ch.left = Val::Px(0.0);
            }
        }
        pl.0 = Some(li);
    }
}

fn on_outside_click(
    _trigger: Trigger<Pointer<Click>>,
    mut opened_popup: ResMut<OpenedPopup>,
    mut node_query: Query<&mut Node>,
) {
    info!("Outside click detected!");
    if let Some(popup) = opened_popup.take() {
        let mut node = node_query
            .get_mut(popup)
            .expect("UI nodes should contain Node");
        node.display = Display::None;
    }
}

pub fn spawn_settings(mut commands: Commands, ui_assets: Res<UIAssets>, ui_style: Res<UiStyle>) {
    let root_entity = vertically_centered(
        &mut commands,
        (
            BackgroundColor(BLANCHED_ALMOND.into()),
            Name::new("VertLayout"),
        ),
        NodeModifier::root(),
        |parent| {
            grid_hor_center_layout(parent, (), NodeModifier::new(), 2, |parent| {
                spawn_combobox(
                    &mut *parent,
                    DropdownMenu {
                        items: vec![
                            DropdownMenuItem {
                                label: "position 1".to_string(),
                            },
                            DropdownMenuItem {
                                label: "position 2".to_string(),
                            },
                            DropdownMenuItem {
                                label: "position 3".to_string(),
                            },
                        ],
                    },
                    NodeModifier::new().set_grid_column(GridPlacement::start_span(2, 1)),
                    ui_assets,
                    ui_style,
                );
                parent.spawn((Text("Hello world\nIn this uncertain times,\nthe world becomes more dangerous than before".to_string()), 
                    Node {
                        grid_column: GridPlacement::start_span(2, 1),
                        ..Default::default()
                    }));
            });
        },
    );
    commands.entity(root_entity).observe(on_outside_click);
}

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_settings)
            .insert_resource(OpenedPopup(None))
            .add_systems(Update, on_combobutton_layout);
    }
}
