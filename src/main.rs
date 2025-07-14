use bevy::{color::palettes::css::*, prelude::*};
use main_menu::{MainMenu, MainMenuAction, MainMenuEvent, MainMenuItem};
mod main_menu;

#[derive(States, Clone, Eq, PartialEq, Hash, Debug, Default)]
enum AppState {
    #[default]
    MainMenu,
    // Game,
}

fn spawn_camera(mut comms: Commands) {
    comms.spawn(Camera2d);
}

fn on_menu_event(mut events: EventReader<MainMenuEvent>, mut exit_events: EventWriter<AppExit>) {
    for eve in events.read() {
        let data = eve.data;
        info!("Main menu event received, data {data}");
        // Exit when user requests to quit.
        if data == 4 {
            exit_events.write(AppExit::Success);
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(main_menu::MainMenuPlugin {
            menu_state: AppState::MainMenu,
            menu: MainMenu(vec![
                MainMenuItem {
                    label: "Start".to_string(),
                    action: MainMenuAction::SubMenu(vec![
                        MainMenuItem {
                            label: "New game".to_string(),
                            action: MainMenuAction::SendEvent(1),
                        },
                        MainMenuItem {
                            label: "Load game...".to_string(),
                            action: MainMenuAction::SendEvent(2),
                        },
                        MainMenuItem {
                            label: "Go back...".to_string(),
                            action: MainMenuAction::GoBack,
                        },
                    ]),
                },
                MainMenuItem {
                    label: "Settings".to_string(),
                    action: MainMenuAction::SendEvent(3),
                },
                MainMenuItem {
                    label: "Quit".to_string(),
                    action: MainMenuAction::SendEvent(4),
                },
            ]),
        })
        .init_state::<AppState>()
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, on_menu_event.run_if(on_event::<MainMenuEvent>))
        .insert_resource(ClearColor(VIOLET.into()))
        .run();
}
