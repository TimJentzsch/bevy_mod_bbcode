use bevy::{asset::AssetPath, prelude::*};

use super::{conversion::convert_bbcode, font::FontPlugin};

pub struct BbcodePlugin {
    font_folder_path: AssetPath<'static>,
}

impl BbcodePlugin {
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
        let _ = asset_server.load_folder(self.font_folder_path.clone());
    }
}
