use std::{ops::Deref, sync::Arc};

use ab_glyph::Font as _;
use bevy::{prelude::*, utils::HashMap};
use tinyvec::TinyVec;

#[derive(Debug)]
pub struct FontPlugin;

impl Plugin for FontPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FontRegistry>()
            .add_systems(Update, update_font_registry);
    }
}

#[derive(Debug, Default, Resource)]
pub struct FontRegistry {
    /// A mapping from asset IDs to font IDs.
    ///
    /// Needed to properly clean up removed fonts.
    asset_to_font_id: HashMap<AssetId<Font>, TinyVec<[fontdb::ID; 8]>>,

    /// A mapping from font IDs to asset IDs.
    ///
    /// Needed to determine which handle to use for a font query.
    font_to_asset_id: HashMap<fontdb::ID, AssetId<Font>>,

    /// The internal database used to query fonts.
    font_db: fontdb::Database,
}

impl FontRegistry {
    /// Add the font associated with the given asset ID.
    pub fn add(&mut self, asset_id: AssetId<Font>, font_assets: impl Deref<Target = Assets<Font>>) {
        let Some(font) = font_assets.get(asset_id) else {
            return;
        };

        let data = font.font.font_data().to_vec();

        // Insert the font into the DB
        let font_ids = self
            .font_db
            .load_font_source(fontdb::Source::Binary(Arc::new(data)));

        // Update the ID maps
        for font_id in &font_ids {
            self.font_to_asset_id.insert(*font_id, asset_id);
        }
        self.asset_to_font_id.insert(asset_id, font_ids);
    }

    /// Remove the font associated with the given asset ID.
    pub fn remove(&mut self, asset_id: AssetId<Font>) {
        let Some(font_ids) = self.asset_to_font_id.get(&asset_id) else {
            return;
        };

        // Remove the font from the DB
        for font_id in font_ids {
            self.font_db.remove_face(*font_id);
        }

        // Update the ID maps
        for font_id in font_ids {
            self.font_to_asset_id.remove(font_id);
        }
        self.asset_to_font_id.remove(&asset_id);
    }

    /// Update the font associated with the given asset ID.
    pub fn update(
        &mut self,
        asset_id: AssetId<Font>,
        font_assets: impl Deref<Target = Assets<Font>>,
    ) {
        self.remove(asset_id);
        self.add(asset_id, font_assets);
    }

    /// Find the best matching font asset for the query.
    pub fn query(&self, query: &fontdb::Query) -> Option<AssetId<Font>> {
        let font_id = self.font_db.query(query);
        font_id
            .and_then(|font_id| self.font_to_asset_id.get(&font_id))
            .copied()
    }
}

/// Track when fonts are loaded, modified or removed and update the font registry accordingly.
fn update_font_registry(
    mut font_registry: ResMut<FontRegistry>,
    mut font_changes: EventReader<AssetEvent<Font>>,
    font_assets: Res<Assets<Font>>,
) {
    for change in font_changes.read() {
        match *change {
            AssetEvent::Added { id } => font_registry.add(id, font_assets.as_ref()),
            AssetEvent::Modified { id } => font_registry.update(id, font_assets.as_ref()),
            AssetEvent::Removed { id } => font_registry.remove(id),
            AssetEvent::Unused { id: _ } => {}
            AssetEvent::LoadedWithDependencies { id: _ } => {}
        }
    }
}
