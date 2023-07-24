#![allow(unused)]

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::winit::{WinitSettings, WinitWindows};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use rs::{Item, Recipe};

pub mod desc_win;
pub mod gemw;
pub mod rs;
mod search;
mod stub;

fn main() {
    App::new()
        .add_plugins(stub::Plugin())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Herbicalc".into(),
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin)
        .add_plugins(pecs::prelude::PecsPlugin)
        .add_plugins(gemw::Plugin)
        .insert_resource(WinitSettings::desktop_app())
        .add_event::<desc_win::WinEvent>()
        .add_systems(Update, desc_win::item_windows)
        .add_systems(Update, desc_win::recipe_windows)
        .add_systems(PostUpdate, desc_win::process_events)
        .add_systems(Update, search::index_new::<Item>)
        .add_systems(Update, search::update_index::<Item>)
        .add_systems(Update, search::search)
        .init_resource::<search::State>()
        .add_systems(
            Update,
            (search::index_new::<Recipe>, search::update_index::<Recipe>),
        )
        .add_systems(PostUpdate, search::update_results)
        .run();
}
