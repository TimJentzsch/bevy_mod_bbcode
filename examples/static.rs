use bevy::prelude::*;
use bevy_mod_bbcode::bevy::{
    bbcode::{BbcodeBundle, BbcodeSettings},
    plugin::BbcodePlugin,
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, BbcodePlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(BbcodeBundle::from_content(
        "test",
        BbcodeSettings {
            regular_font: asset_server.load("fonts/FiraSans-Regular.ttf"),
            bold_font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            italic_font: asset_server.load("fonts/FiraSans-Italic.ttf"),
        },
    ));
}
