# bevy_mod_bbcode

Rich text support in Bevy using a custom [BBCode](https://en.wikipedia.org/wiki/BBCode) markup flavor.

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
        // Register the font files stored in `assets/fonts`
        .add_plugins((DefaultPlugins, BbcodePlugin::new().with_fonts("fonts")))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(BbcodeBundle::from_content(
        "test [b]bold[/b] with [i]italic[/i] and [c=#ff00ff]color[/c]",
        // Use the "Fira Sans" font family with a default font size of 40
        BbcodeSettings::new("Fira Sans", 40., Color::WHITE),
    ));
}
```

See `examples` for more usage patterns!

### Supported Tags

- `b`: \[b]**bold**\[/b] text
- `i`: \[i]_italic_\[/i] text
- `c`: \[c=\#ff0000]<span style="color: red">colored</span>\[/c] text
  - Register named colors via `ResMut<ColorMap>` and use the names instead of hex values
- `m`: \[m=foo]text with marker component\[/m]
  - Register marker components via `BbcodeSettings::with_marker` and use them to update text dynamically
- `font`: \[font="Fira Sans"]change the font family\[/font]

## License

This project is licensed under the terms of the [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE) license at your choice.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

Note that the assets used in the examples might use different licenses, see [`assets/CREDITS.md`](assets/CREDITS.md).
