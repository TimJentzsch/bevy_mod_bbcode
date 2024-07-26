//! This example demonstrates how parts of the text can be efficiently updated dynamically.
//! To do this, we use the special `[m]` tag, which allows us to assign a marker component to the contained text.
//! We can then query for the marker component as usual and apply our edits.

use bevy::prelude::*;
use bevy_mod_bbcode::{BbcodeBundle, BbcodePlugin, BbcodeSettings};

#[derive(Component, Clone)]
struct TimeMarker;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, BbcodePlugin::new("fonts")))
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(BbcodeBundle::from_content(
        "Time passed: [m=time]0.0[/m] s",
        BbcodeSettings::new(40., Color::WHITE)
            .with_regular_font(asset_server.load("fonts/FiraSans-Regular.ttf"))
            .with_bold_font(asset_server.load("fonts/FiraSans-Bold.ttf"))
            .with_italic_font(asset_server.load("fonts/FiraSans-Italic.ttf"))
            // Register the marker component for the `m=time` tag
            .with_marker("time", TimeMarker),
    ));
}

fn update(time: Res<Time>, mut query: Query<&mut Text, With<TimeMarker>>) {
    for mut text in query.iter_mut() {
        // We can directly query for the `Text` component and update it, without the BBCode being parsed again
        text.sections[0].value = format!("{:.0}", time.elapsed_seconds());
    }
}
