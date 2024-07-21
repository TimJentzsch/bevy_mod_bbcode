use bevy::prelude::*;
use bevy_mod_bbcode::bevy::{
    bbcode::{Bbcode, BbcodeBundle, BbcodeSettings},
    plugin::BbcodePlugin,
};

#[derive(Debug, Component)]
struct Marker;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, BbcodePlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        BbcodeBundle::from_content(
            // The text will be added later
            "",
            BbcodeSettings {
                regular_font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                bold_font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                italic_font: asset_server.load("fonts/FiraSans-Italic.ttf"),

                font_size: 40.,
                color: Color::WHITE,
            },
        ),
        Marker,
    ));
}

fn update(time: Res<Time>, mut query: Query<(&mut Bbcode, &mut BbcodeSettings), With<Marker>>) {
    let (mut bbcode, mut settings) = query.single_mut();

    // Dynamically change the text to the elapsed time
    bbcode.content = format!(
        "Time passed: [b][c=#ffffff]{:.2}[/c][/b]",
        time.elapsed_seconds()
    );
    // Dynamically change the default text color
    settings.color = Hsla::hsl((time.elapsed_seconds() * 20.) % 360., 1., 0.7).into();
}
