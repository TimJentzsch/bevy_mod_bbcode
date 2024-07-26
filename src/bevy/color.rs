use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};

pub struct ColorPlugin;

impl Plugin for ColorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ColorMap>()
            .add_systems(Update, update_colors);
    }
}

#[derive(Debug, Resource, Default)]
pub struct ColorMap {
    /// The map from name to color.
    map: HashMap<String, Color>,

    /// Internal tracker for names where the corresponding color has been updated.
    ///
    /// Used to only update what's needed.
    was_updated: HashSet<String>,
}

impl ColorMap {
    /// Insert (add or update) a new named color.
    ///
    /// Returns `&mut self` for chaining.
    pub fn insert<N, C>(&mut self, name: N, color: C) -> &mut Self
    where
        N: Into<String>,
        C: Into<Color>,
    {
        self.map.insert(name.into(), color.into());
        self
    }

    /// Get the color for the given name.
    pub fn get(&self, name: &str) -> Option<Color> {
        self.map.get(name).copied()
    }

    /// Determine if any color has been updated.
    pub(crate) fn has_update(&self) -> bool {
        !self.was_updated.is_empty()
    }

    /// Determine if the color with the given name has been updated, and if yes to which value.
    ///
    /// You should probably call [`ColorMap::clear_was_updated`] at some point afterwards.
    pub(crate) fn get_update(&self, name: &str) -> Option<Color> {
        if self.was_updated.contains(name) {
            self.map.get(name).copied()
        } else {
            None
        }
    }

    /// Clear the tracker for the color names which had their values updated.
    pub(crate) fn clear_was_updated(&mut self) {
        self.was_updated.clear();
    }
}

/// Tracker for text that's colored via named BBCode components.
#[derive(Debug, Component)]
pub struct BbCodeColored {
    pub name: String,
}

/// Update all colors whose name has changed.
fn update_colors(
    mut color_map: ResMut<ColorMap>,
    mut colored_text_query: Query<(&BbCodeColored, &mut Text)>,
) {
    if !color_map.is_changed() || !color_map.has_update() {
        return;
    }

    for (colored, mut text) in colored_text_query.iter_mut() {
        if let Some(color) = color_map.get_update(&colored.name) {
            for section in &mut text.sections {
                section.style.color = color;
            }
        }
    }

    color_map.clear_was_updated();
}
