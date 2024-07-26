use bevy::prelude::*;

use super::registry::FontRegistry;

#[derive(Debug)]
pub struct FontPlugin;

impl Plugin for FontPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FontRegistry>()
            .add_systems(Update, update_font_registry);
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
