mod modules;
mod rendering;

use crate::structs::{
    PlayerController,
    MapController,
    CubemapController,
    GunController,
    EntityHandler,
    AudioController
};
use bevy_scene_hook::HookPlugin;
use bevy_rapier3d::prelude::*;
use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::*,
    window::{
        Cursor,
        CursorGrabMode,
        PresentMode, 
        WindowMode,
        WindowResolution,
        WindowTheme
    }
};
use modules::{
    game, 
    controls,
    audio,
    gunplay,
    structs,
    menu
};
use rendering::{
    entities
};

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Resource, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    PauseMenu,
    Playing,
}

fn main() {
    App::new()
    .add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "VALOFFBRAND 2.0".into(),
                mode: WindowMode::BorderlessFullscreen,
                resolution: WindowResolution::new(1920., 1080.),
                present_mode: PresentMode::AutoNoVsync,
                window_theme: Some(WindowTheme::Dark),
                cursor: Cursor { 
                    icon: default(),
                    visible: (true),
                    grab_mode: (CursorGrabMode::None),
                    hit_test: (true)
                },
                enabled_buttons: bevy::window::EnabledButtons {
                    maximize: false,
                    ..Default::default()
                },
                visible: true,
                ..default()
            }),
            exit_condition: bevy::window::ExitCondition::OnPrimaryClosed,
            close_when_requested: true,
            ..default()
        }),
        FrameTimeDiagnosticsPlugin,
        RapierPhysicsPlugin::<NoUserData>::default(),
        HookPlugin,
    ))
    .insert_state(GameState::MainMenu)
    .insert_resource(Msaa::Sample8)
    .init_resource::<PlayerController>() 
    .init_resource::<MapController>()
    .init_resource::<GunController>()  
    .init_resource::<AudioController>()
    .init_resource::<CubemapController>()
    .init_resource::<EntityHandler>()
    .add_systems(OnEnter(GameState::MainMenu), menu::setup_main_menu)
    .add_systems(Update, menu::menu_interactions.run_if(game::in_main_menu_state))
    .add_systems(OnEnter(GameState::PauseMenu), menu::setup_pause_menu)
    .add_systems(Update, menu::menu_interactions.run_if(game::in_pause_menu_state))
    .add_systems(OnTransition {
        from: GameState::MainMenu, 
        to: GameState::Playing
    }, (
        game::setup,
        entities::setup,
    ))
    .add_systems(Update, (
        game::mouse_callback,
        entities::rotate_map,
        entities::rotate_gun,
        entities::load_cubemap,
        controls::update,
        gunplay::update,
        audio::audio_playback,
        audio::audio_control
    ).run_if(game::in_playing_state))
    .add_systems(OnExit(GameState::MainMenu), entities::despawn_menu_entities)
    .add_systems(OnExit(GameState::PauseMenu), entities::despawn_menu_entities)
    .add_systems(OnTransition {
        from: GameState::PauseMenu,
        to: GameState::MainMenu
    }, entities::despawn_game_entities)
    .add_systems(Startup, (entities::load_entities, audio::load_audio))
    .add_systems(OnTransition {
        from: GameState::PauseMenu,
        to: GameState::Playing
    }, game::change_cursor_state)
    .run();
}