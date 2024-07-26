use bevy::prelude::*;
use bevy_mod_bbcode::{BbcodeBundle, BbcodePlugin, BbcodeSettings};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, BbcodePlugin::new("fonts")))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(BbcodeBundle::from_content(
        "test [b]bold[/b] with [i]italic[/i] and [c=#ff00ff]color[/c]",
        BbcodeSettings::new(40., Color::WHITE)
            .with_regular_font(asset_server.load("fonts/FiraSans-Regular.ttf"))
            .with_bold_font(asset_server.load("fonts/FiraSans-Bold.ttf"))
            .with_italic_font(asset_server.load("fonts/FiraSans-Italic.ttf")),
    ));
}
