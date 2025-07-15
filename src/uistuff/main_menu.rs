use bevy::prelude::*;

pub struct MainMenuPlugin<S: States, D>
where
    D: Send + Sync + Clone + 'static,
{
    pub menu_state: S,
    pub menu: MainMenu<D>,
}

#[derive(Event)]
pub struct MainMenuEvent<D>
where
    D: Sync + Clone + 'static + Send,
{
    pub data: D,
}

#[derive(Resource, Clone)]
pub enum MainMenuAction<D>
where
    D: Send + Sync + Clone + 'static,
{
    SubMenu(Vec<MainMenuItem<D>>),
    SendEvent(D),
    GoBack,
}

#[derive(Resource, Clone)]
pub struct MainMenuItem<D>
where
    D: Send + Sync + Clone + 'static,
{
    pub label: String,
    pub action: MainMenuAction<D>,
}

#[derive(Resource, Clone)]
pub struct MainMenu<D: Send + Sync + Clone + 'static>(pub Vec<MainMenuItem<D>>);

mod internal {
    use super::{MainMenuAction, MainMenuEvent, MainMenuItem};
    use crate::uistuff::layouts::*;
    use std::path::Path;

    use bevy::prelude::*;

    #[derive(Clone)]
    struct MenuState<D>
    where
        D: Send + Sync + Clone + 'static,
    {
        current_menu: Vec<MainMenuItem<D>>,
    }

    #[derive(Resource, Clone)]
    struct MenuStateResource<D>
    where
        D: Send + Sync + Clone + 'static,
    {
        state_stack: Vec<MenuState<D>>,
        current_state_idx: usize,
    }

    #[derive(Component)]
    struct MenuRoot;

    #[derive(Resource, Default)]
    struct MenuAssets {
        font: Handle<Font>,
    }

    fn load_assets(fonts: Res<AssetServer>, mut assets: ResMut<MenuAssets>) {
        info!("loading assets..");
        let gamefont = fonts.load::<Font>(Path::new("fonts/Beholden-Medium.ttf"));
        info!("Font handle in load_assets is {gamefont:?}");
        assets.font = gamefont;
    }

    #[derive(Event)]
    enum InternalMenuEvent<D>
    where
        D: Send + Sync + Clone + 'static,
    {
        GoBack,
        OpenMenu(Vec<MainMenuItem<D>>),
    }

    type MainMenuEventWriter<'a, D> = EventWriter<'a, MainMenuEvent<D>>;
    type InternalMenuEventWriter<'a, D> = EventWriter<'a, InternalMenuEvent<D>>;
    fn mouseclick_observer<D: Sync + Clone + 'static + Send>(
        item_action: MainMenuAction<D>,
    ) -> impl Fn(Trigger<Pointer<Click>>, MainMenuEventWriter<D>, InternalMenuEventWriter<D>) {
        move |_trigger, mut event_sender, mut internal_sender| match item_action {
            MainMenuAction::SendEvent(ref d) => {
                event_sender.write(MainMenuEvent { data: d.clone() });
            }
            MainMenuAction::SubMenu(ref items) => {
                internal_sender.write(InternalMenuEvent::OpenMenu(items.clone()));
            }
            MainMenuAction::GoBack => {
                internal_sender.write(InternalMenuEvent::GoBack);
            }
        }
    }

    fn on_internal_menu_event<D: Sync + Clone + 'static + Send>(
        mut menu_state: ResMut<MenuStateResource<D>>,
        mut internal_event: EventReader<InternalMenuEvent<D>>,
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

    fn rebuild_menu<D: Sync + Clone + 'static + Send>(
        mut command: Commands,
        assets: Res<MenuAssets>,
        menu_state: Res<MenuStateResource<D>>,
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

    fn init_menu<D: Sync + Clone + 'static + Send>(
        command: Commands,
        assets: Res<MenuAssets>,
        menu_data: Res<MenuStateResource<D>>,
    ) {
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

    impl<D: Sync + Clone + Send, S: States> Plugin for super::MainMenuPlugin<S, D> {
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
                    init_menu::<D>.after(load_assets),
                )
                .add_systems(
                    Update,
                    rebuild_menu::<D>
                        .run_if(in_state(self.menu_state.clone()))
                        .run_if(resource_exists_and_changed::<MenuStateResource<D>>),
                )
                .add_systems(
                    Update,
                    on_internal_menu_event::<D>
                        .run_if(in_state(self.menu_state.clone()))
                        .run_if(on_event::<InternalMenuEvent<D>>)
                        .before(rebuild_menu::<D>),
                )
                // .add_systems(Startup, spawn_text)
                .init_resource::<MenuAssets>()
                .add_event::<InternalMenuEvent<D>>()
                .add_event::<MainMenuEvent<D>>()
                .insert_resource(starting_menu_state)
                .insert_resource(self.menu.clone());
        }
    }
}
