use bevy::prelude::*;
use bevy_mod_bbcode::{Bbcode, BbcodePlugin, BbcodeSettings};

fn main() {
    App::new()
        // Register the font files stored in `assets/fonts`
        .add_plugins((DefaultPlugins, BbcodePlugin::new().with_fonts("fonts")))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((Bbcode::new(
        r#"test [b]bold with [i]italic[/i][/b] and [c=#ff00ff]color[/c] and [font="JetBrains Mono"]different font[/font]"#),
        // Use the "Fira Sans" font family with a default font size of 40
        BbcodeSettings::new("Fira Sans", 40., Color::WHITE),
    ));
}
