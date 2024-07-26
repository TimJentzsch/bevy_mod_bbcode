use bevy::{
    asset::{AssetPath, LoadedFolder},
    prelude::*,
};

use super::{conversion::convert_bbcode, font::FontPlugin};

#[derive(Debug, Default)]
pub struct BbcodePlugin {
    /// The path to a folder containing the fonts to use.
    font_folder_path: Option<AssetPath<'static>>,
}

impl BbcodePlugin {
    /// Create a new BBCode plugin.
    ///
    /// You probably also want to load fonts to make the text formatting work, see [`BbcodePlugin::with_fonts`].
    pub fn new() -> Self {
        Self {
            font_folder_path: None,
        }
    }

    /// Load the fonts in the given directory.
    /// The directory should only contain font files and nothing else.
    pub fn with_fonts<P: Into<AssetPath<'static>>>(mut self, folder_path: P) -> Self {
        self.font_folder_path = Some(folder_path.into());
        self
    }
}

impl Plugin for BbcodePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FontPlugin)
            .add_systems(Update, convert_bbcode);

        let asset_server = app.world().resource::<AssetServer>();

        if let Some(folder_path) = &self.font_folder_path {
            // Load all fonts in the provided folder
            // `FontPlugin` will react to the asset events
            let handle = asset_server.load_folder(folder_path.clone());
            // We still need to store the handle to keep the fonts loaded
            app.insert_resource(FontFolder { _handle: handle });
        }
    }
}

#[derive(Debug, Resource)]
struct FontFolder {
    /// Keep the assets loaded by storing the strong handle
    _handle: Handle<LoadedFolder>,
}
