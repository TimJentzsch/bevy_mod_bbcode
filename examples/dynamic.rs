//! This example demonstrates how parts of the text can be efficiently updated dynamically.
//!
//! - To update the text content, we use the `[m]` tag.
//!   It allows us to assign a marker component to the contained text,
//!   which we can then update using queries as usual.
//! - To update the text color, we use the `[c]` tag with named colors.
//!   We simply update the color for the given name and it updates everywhere.

use bevy::prelude::*;
use bevy_mod_bbcode::{Bbcode, BbcodePlugin, BbcodeSettings, ColorMap};

#[derive(Component, Clone)]
struct TimeMarker;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, BbcodePlugin::new().with_fonts("fonts")))
        .add_systems(Startup, setup)
        .add_systems(Update, (update_text, update_color))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Bbcode::new("Time passed: [m=time]0.0[/m] s with [c=rainbow]rainbow[/c]"),
        BbcodeSettings::new("Fira Sans", 40., Color::WHITE)
            // Register the marker component for the `m=time` tag
            .with_marker("time", TimeMarker),
    ));
}

fn update_text(time: Res<Time>, mut query: Query<&mut TextSpan, With<TimeMarker>>) {
    for mut text in query.iter_mut() {
        // We can directly query for the `TextSpan` component and update it, without the BBCode being parsed again
        *text = format!("{:.0}", time.elapsed_secs()).into();
    }
}

fn update_color(time: Res<Time>, mut color_map: ResMut<ColorMap>) {
    let hue = (time.elapsed_secs() * 20.) % 360.;
    // Updating a value in the color map will update that color wherever the same name is used!
    color_map.insert("rainbow", Hsva::hsv(hue, 1., 1.));
}
