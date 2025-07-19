use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use uistuff::config::STANDARD_STYLE;
use uistuff::main_menu::{MainMenu, MainMenuAction, MainMenuEvent, MainMenuItem, MainMenuPlugin};
use uistuff::settings::SettingsPlugin;
use uistuff::utils::UiUtilsPlugin;
mod uistuff;

#[derive(States, Clone, Eq, PartialEq, Hash, Debug, Default)]
enum AppState {
    #[default]
    MainMenu,
    // Game,
}

fn spawn_camera(mut comms: Commands) {
    comms.spawn(Camera2d);
}

fn on_menu_event(
    mut events: EventReader<MainMenuEvent<u32>>,
    mut exit_events: EventWriter<AppExit>,
) {
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
        .add_plugins(UiUtilsPlugin)
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(SettingsPlugin)
        // .add_plugins(MainMenuPlugin {
        //     menu_state: AppState::MainMenu,
        //     menu: MainMenu(vec![
        //         MainMenuItem {
        //             label: "Start".to_string(),
        //             action: MainMenuAction::SubMenu(vec![
        //                 MainMenuItem {
        //                     label: "New game".to_string(),
        //                     action: MainMenuAction::SendEvent(1u32),
        //                 },
        //                 MainMenuItem {
        //                     label: "Load game...".to_string(),
        //                     action: MainMenuAction::SendEvent(2u32),
        //                 },
        //                 MainMenuItem {
        //                     label: "Go back...".to_string(),
        //                     action: MainMenuAction::GoBack,
        //                 },
        //             ]),
        //         },
        //         MainMenuItem {
        //             label: "Settings".to_string(),
        //             action: MainMenuAction::SendEvent(3u32),
        //         },
        //         MainMenuItem {
        //             label: "Quit".to_string(),
        //             action: MainMenuAction::SendEvent(4u32),
        //         },
        //     ]),
        // })
        .init_state::<AppState>()
        .add_systems(Startup, spawn_camera)
        // .add_systems(Update, on_menu_event.run_if(on_event::<MainMenuEvent<u32>>))
        .insert_resource(ClearColor(STANDARD_STYLE.back_color))
        .insert_resource(STANDARD_STYLE)
        .run();
}
