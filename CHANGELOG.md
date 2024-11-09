# Changelog

## v0.3.0-rc.1

This release updates to Bevy 0.15, which reworks text and introduces required components.

### No More Bundles

With required components, we no longer need the `BbcodeBundle`.
Instead, just spawn the `Bbcode` and `BbcodeSettings` components.
All other needed components will be inserted automatically.

### Query for `TextSpan`

When you wanted to update text dynamically via marker components inserted via BBcode, you used to query for the marker component and `Text`.
Now, query for `TextSpan` instead, which is what the BBcode markup gets parsed into now.

## v0.2.0

This release simplifies font handling, adds named colors and adds support for efficiently changing text dynamically.

### Simplified Font Handling

In the past, you needed to pass around multiple `Handle<Font>`s for each variant of a font (e.g. bold, italic, bold and italic, etc.).

Instead, we now use a `FontRegistry` which automatically tracks which `Font`s are loaded and allows to query them by font weight, style and other factors.
You can now simply specify an asset path where your fonts are stored and they will all be loaded and used inside of BBCode!

```rs
// Load the fonts inside of assets/fonts
BbcodePlugin::new().with_fonts("fonts")
```

Then inside of `BbcodeSettings`, you just specify the font family, size and color of the text to use by default:

```rs
BbcodeSettings::new("Fira Sans", 40., Color::WHITE)
```

Additionally, the new `font` tag allows you to change the font family for parts of the text:

```txt
[font="JetBrains Mono"]new font family[/font]
```

### Named Colors

The `color`/`c` tag now also supports named colors, e.g. `[c=primary]text[/c]`.
The color values are specified in the new `ColorMap` resource.

Changing a value in the color map resource will dynamically update all occurrences of this color in your app!

### Dynamic Text Editing

Until this point, you could dynamically change the text in your app by changing `Bbcode.content`.
However, this adds a performance overhead as the entire markup needs to be parsed again and the UI hierarchy reconstructed.

Instead, you can now register marker components:

```rs
#[derive(Component, Clone)]
struct TimeMarker;

BbcodeSettings::new("Fira Sans", 40., Color::WHITE)
    // Register the marker component
    .with_marker("time", TimeMarker)
```

And then use it with the new `m` tag:

```txt
Time passed: [m=time]0.0[/m]
```

Finally, you can use queries to efficiently update the text:

```rs
fn update_text(time: Res<Time>, mut query: Query<&mut Text, With<TimeMarker>>) {
    for mut text in query.iter_mut() {
        text.sections[0].value = format!("{:.0}", time.elapsed_seconds());
}
```

## Migration Guide

The `BbcodePlugin` now has a constructor and allows you to load an entire folder of fonts:

```rs
// Load the fonts inside of assets/fonts
BbcodePlugin::new().with_fonts("fonts")
```

In the `BbcodeSettings` you now specify the font family, size and color of the default text:

```rs
BbcodeSettings::new("Fira Sans", 40., Color::WHITE)
```

## v0.1.0

The initial release!

Supports basic BBCode formatting with bold, italic and colored text.
