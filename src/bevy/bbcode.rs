use std::sync::Arc;

use bevy::{ecs::system::EntityCommands, prelude::*, utils::HashMap};

use super::color::BbCodeColor;

#[derive(Debug, Clone, Component, Default)]
#[require(Text, BbcodeSettings)]

pub struct Bbcode {
    /// The bbcode-formatted text.
    pub content: String,
}

impl Bbcode {
    /// Create a new Bbcode text with the given content in the Bbcode Markup language.
    pub fn new<S: Into<String>>(content: S) -> Self {
        Self {
            content: content.into(),
        }
    }
}

type ModifierFn = dyn Fn(&mut EntityCommands) + Send + Sync;

#[derive(Clone, Default)]
pub(crate) struct Modifiers {
    pub(crate) modifier_map: HashMap<String, Arc<ModifierFn>>,
}

#[derive(Clone, Component)]
pub struct BbcodeSettings {
    pub font_family: String,
    pub font_size: f32,
    pub color: BbCodeColor,

    pub(crate) modifiers: Modifiers,
}

impl BbcodeSettings {
    pub fn new<F: Into<String>, C: Into<BbCodeColor>>(
        font_family: F,
        font_size: f32,
        color: C,
    ) -> Self {
        Self {
            font_family: font_family.into(),
            font_size,
            color: color.into(),
            modifiers: Default::default(),
        }
    }

    /// Register a marker component for the `[m]` tag.
    pub fn with_marker<N: Into<String>, M: Component + Clone>(
        mut self,
        tag_name: N,
        marker: M,
    ) -> Self {
        self.modifiers.modifier_map.insert(
            tag_name.into(),
            Arc::new(move |commands| {
                commands.insert(marker.clone());
            }),
        );
        self
    }
}

impl Default for BbcodeSettings {
    fn default() -> Self {
        Self {
            color: Color::WHITE.into(),
            // TODO: Revisit what to put as default here
            font_family: Default::default(),
            font_size: 20.0,
            modifiers: Default::default(),
        }
    }
}
