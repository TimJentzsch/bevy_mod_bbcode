use bevy::{
    asset::{AssetPath, LoadedFolder},
    prelude::*,
};

use super::{conversion::convert_bbcode, font::FontPlugin};

pub struct BbcodePlugin {
    /// The path to a folder containing the fonts to use.
    font_folder_path: AssetPath<'static>,
}

impl BbcodePlugin {
    /// Create a new BBCode plugin, using fonts from the given folder.
    pub fn new<P: Into<AssetPath<'static>>>(font_folder_path: P) -> Self {
        Self {
            font_folder_path: font_folder_path.into(),
        }
    }
}

impl Plugin for BbcodePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FontPlugin)
            .add_systems(Update, convert_bbcode);

        let asset_server = app.world().resource::<AssetServer>();

        // Load all fonts in the provided folder
        // `FontPlugin` will react to the asset events
        let handle = asset_server.load_folder(self.font_folder_path.clone());
        // We still need to store the handle to keep the fonts loaded
        app.insert_resource(FontFolder { _handle: handle });
    }
}

#[derive(Debug, Resource)]
struct FontFolder {
    /// Keep the assets loaded by storing the strong handle
    _handle: Handle<LoadedFolder>,
}
