use std::ops::Deref;

use bevy::{prelude::*, utils::HashMap};
use tinyvec::TinyVec;

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

        let data = font.data.clone();

        // Insert the font into the DB
        let font_ids = self.font_db.load_font_source(fontdb::Source::Binary(data));

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

    /// Find the best matching font asset for the query and return its [`AssetId`].
    pub fn query_id(&self, query: &fontdb::Query) -> Option<AssetId<Font>> {
        let font_id = self.font_db.query(query);
        font_id
            .and_then(|font_id| self.font_to_asset_id.get(&font_id))
            .copied()
    }

    /// Find the best matching font asset for the query and return its [`Handle`].
    ///
    /// Note that this returns a *weak* handle to the font.
    pub fn query_handle(&self, query: &fontdb::Query) -> Option<Handle<Font>> {
        self.query_id(query).map(Handle::Weak)
    }
}
