use bevy::prelude::*;

#[derive(Component)]
pub struct MainMenu;

#[derive(Component)]
pub struct MainMenuCamera;

// === Buttons ===
#[derive(Component)]
pub struct NewGameButton;

#[derive(Component)]
pub struct LoadGameButton;

#[derive(Component)]
pub struct OptionsButton;

#[derive(Component)]
pub struct QuitButton;
