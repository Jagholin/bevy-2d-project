use std::path::Path;

use bevy::color::palettes::css::*;
use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::prelude::*;

pub type MainMenuEventData = u32;

#[derive(Event)]
pub struct MainMenuEvent {
    pub data: MainMenuEventData,
}

#[derive(Resource, Clone)]
pub enum MainMenuAction {
    SubMenu(Vec<MainMenuItem>),
    SendEvent(MainMenuEventData),
    GoBack,
}

#[derive(Resource, Clone)]
pub struct MainMenuItem {
    pub label: String,
    pub action: MainMenuAction,
}

#[derive(Resource, Clone)]
pub struct MainMenu(pub Vec<MainMenuItem>);

#[derive(Clone)]
struct MenuState {
    current_menu: Vec<MainMenuItem>,
}

#[derive(Resource, Clone)]
struct MenuStateResource {
    state_stack: Vec<MenuState>,
    current_state_idx: usize,
}

#[derive(Component)]
struct MenuRoot;

#[derive(Clone, Default, Debug)]
struct NodeModifier {
    grid_column: Option<GridPlacement>,
    grid_row: Option<GridPlacement>,
}

impl NodeModifier {
    fn new() -> Self {
        NodeModifier {
            ..Default::default()
        }
    }
    fn set_grid_column(mut self, pl: GridPlacement) -> Self {
        self.grid_column = Some(pl);
        self
    }
    fn set_grid_row(mut self, pl: GridPlacement) -> Self {
        self.grid_row = Some(pl);
        self
    }
    fn modify(&self, n: Node) -> Node {
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

#[derive(Resource, Default)]
struct MenuAssets {
    font: Handle<Font>,
}

#[derive(Component, Debug)]
struct ChangeColorOnHover {
    normal_color: Color,
    hover_color: Color,
}

fn change_color_on_hover(
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

pub struct MainMenuPlugin<S: States> {
    pub menu_state: S,
    pub menu: MainMenu,
}

fn load_assets(fonts: Res<AssetServer>, mut assets: ResMut<MenuAssets>) {
    info!("loading assets..");
    let gamefont = fonts.load::<Font>(Path::new("fonts/Beholden-Medium.ttf"));
    info!("Font handle in load_assets is {gamefont:?}");
    assets.font = gamefont;
}

fn text_box(
    text: impl Into<String>,
    font: Handle<Font>,
    node_modifier: NodeModifier,
) -> impl Bundle {
    (
        node_modifier.modify(Node {
            border: UiRect::all(Val::Px(2.0)),
            padding: UiRect::axes(Val::Auto, Val::Px(5.0)),
            ..Default::default()
        }),
        BackgroundColor(PINK.into()),
        children![(
            Text::new(text),
            TextColor(WHITE.into()),
            TextFont {
                font,
                font_size: 32.0,
                ..Default::default()
            }
        )],
    )
}

fn button_box(
    text: impl Into<String>,
    font: Handle<Font>,
    node_modifier: NodeModifier,
) -> impl Bundle {
    let result = text_box(text, font, node_modifier);
    (
        result,
        ChangeColorOnHover {
            normal_color: PINK.into(),
            hover_color: LIGHT_CORAL.into(),
        },
        Button,
    )
}

fn grid_center_layout(
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

#[derive(Event)]
enum InternalMenuEvent {
    GoBack,
    OpenMenu(Vec<MainMenuItem>),
}

fn mouseclick_observer(
    item_action: MainMenuAction,
) -> impl Fn(Trigger<Pointer<Click>>, EventWriter<MainMenuEvent>, EventWriter<InternalMenuEvent>) {
    move |_trigger, mut event_sender, mut internal_sender| match item_action {
        MainMenuAction::SendEvent(d) => {
            event_sender.write(MainMenuEvent { data: d });
        }
        MainMenuAction::SubMenu(ref items) => {
            internal_sender.write(InternalMenuEvent::OpenMenu(items.clone()));
        }
        MainMenuAction::GoBack => {
            internal_sender.write(InternalMenuEvent::GoBack);
        }
    }
}

fn on_internal_menu_event(
    mut menu_state: ResMut<MenuStateResource>,
    mut internal_event: EventReader<InternalMenuEvent>,
) {
    for event in internal_event.read() {
        match event {
            InternalMenuEvent::GoBack => {
                menu_state.current_state_idx -= 1;
                menu_state.state_stack.pop();
            }
            InternalMenuEvent::OpenMenu(main_menu_items) => {
                menu_state.state_stack.push(MenuState {
                    current_menu: main_menu_items.clone(),
                });
                menu_state.current_state_idx += 1;
            }
        }
    }
}

fn rebuild_menu(
    mut command: Commands,
    assets: Res<MenuAssets>,
    menu_state: Res<MenuStateResource>,
    old_menu_root: Query<Entity, With<MenuRoot>>,
) {
    let mut old_menu_despawned = false;
    for e in old_menu_root {
        command.entity(e).despawn();
        old_menu_despawned = true;
    }
    if old_menu_despawned {
        init_menu(command, assets, menu_state);
    }
}

fn init_menu(command: Commands, assets: Res<MenuAssets>, menu_data: Res<MenuStateResource>) {
    let my_font = assets.font.clone();
    grid_center_layout(command, MenuRoot, |parent| {
        let modifier = NodeModifier::new().set_grid_column(GridPlacement::start_span(2, 1));
        let current_menu = menu_data
            .state_stack
            .get(menu_data.current_state_idx)
            .expect("The state index should point to existing state");
        for (item, idx) in current_menu.current_menu.iter().zip(1..) {
            parent
                .spawn(button_box(
                    item.label.as_str(),
                    my_font.clone(),
                    modifier
                        .clone()
                        .set_grid_row(GridPlacement::start_span(idx, 1)),
                ))
                .observe(mouseclick_observer(item.action.clone()));
        }
    });
}

impl<S: States> Plugin for MainMenuPlugin<S> {
    fn build(&self, app: &mut App) {
        let starting_menu_state = MenuStateResource {
            state_stack: vec![MenuState {
                current_menu: self.menu.0.clone(),
            }],
            current_state_idx: 0,
        };
        app.add_systems(OnEnter(self.menu_state.clone()), load_assets)
            .add_systems(
                OnEnter(self.menu_state.clone()),
                init_menu.after(load_assets),
            )
            .add_systems(
                Update,
                rebuild_menu
                    .run_if(in_state(self.menu_state.clone()))
                    .run_if(resource_exists_and_changed::<MenuStateResource>),
            )
            .add_systems(
                Update,
                on_internal_menu_event
                    .run_if(in_state(self.menu_state.clone()))
                    .run_if(on_event::<InternalMenuEvent>)
                    .before(rebuild_menu),
            )
            .add_systems(
                Update,
                change_color_on_hover.run_if(in_state(self.menu_state.clone())),
            )
            // .add_systems(Startup, spawn_text)
            .init_resource::<MenuAssets>()
            .add_event::<InternalMenuEvent>()
            .add_event::<MainMenuEvent>()
            .insert_resource(starting_menu_state)
            .insert_resource(self.menu.clone());
    }
}
