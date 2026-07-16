mod app_state;
mod domain;
mod fonts;
mod game_data;
mod screens;
mod theme;

use app_state::AppState;
use bevy::asset::AssetPlugin;
use bevy::log::{DEFAULT_FILTER, LogPlugin};
use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(theme::BG))
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin { file_path: "src/assets".into(), ..default() })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bakery Text".into(),
                        resolution: (960u32, 720u32).into(),
                        ..default()
                    }),
                    ..default()
                })
                // ICU4X's bundled segmenter data has no Japanese/Chinese dictionary, so
                // Parley's text layout logs a warning every time it lays out Japanese text.
                .set(LogPlugin {
                    filter: format!("{DEFAULT_FILTER},icu_provider=off"),
                    ..default()
                }),
        )
        .init_resource::<game_data::GameData>()
        .add_systems(Startup, spawn_camera)
        .init_state::<AppState>()
        .add_plugins((screens::TitlePlugin, screens::PlayingPlugin, screens::LostPlugin));

    // `OnEnter(AppState::Title)` fires as part of the initial `StateTransition`,
    // which runs before even `PreStartup`. `Fonts` must therefore exist before
    // `.run()`, not via a `Startup` system (whose `Commands` wouldn't be
    // applied in time).
    let asset_server = app.world().resource::<AssetServer>().clone();
    app.insert_resource(fonts::Fonts::load(&asset_server));

    app.run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
