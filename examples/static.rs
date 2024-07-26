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
        "test [b]bold with [i]italic[/i][/b] and [c=#ff00ff]color[/c]",
        BbcodeSettings::new("Fira Sans", 40., Color::WHITE),
    ));
}
