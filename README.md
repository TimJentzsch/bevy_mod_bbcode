# bevy_mod_bbcode

Use BBCode-formatted text in Bevy.

## Bevy Compatibility

| `bevy` version | `bevy_mod_bbcode` version |
| -------------- | ------------------------- |
| `0.14`         | `0.1`                     |

## Installation

```cli
cargo add bevy_mod_bbcode
```

## Usage

Instead of spawning a `TextBundle`, spawn a `BbcodeBundle`!

```rs
use bevy::prelude::*;
use bevy_mod_bbcode::{BbcodeBundle, BbcodePlugin, BbcodeSettings};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, BbcodePlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(BbcodeBundle::from_content(
        "test [b]bold[/b] with [i]italic[/i] and [c=#ff00ff]color[/c]",
        BbcodeSettings {
            regular_font: asset_server.load("fonts/FiraSans-Regular.ttf"),
            bold_font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            italic_font: asset_server.load("fonts/FiraSans-Italic.ttf"),

            font_size: 40.,
            color: Color::WHITE,
        },
    ));
}
```

See `examples` for more usage patterns!

## License

This project is licensed under the terms of the [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE) license at your choice.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
