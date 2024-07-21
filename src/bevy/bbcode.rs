use bevy::prelude::*;

use crate::bbcode::BbcodeTag;

#[derive(Debug, Clone, Component)]

pub struct Bbcode {
    /// The bbcode-formatted text.
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct BbcodeStyle {
    pub is_bold: bool,
    pub is_italic: bool,
}

impl BbcodeStyle {
    /// Change the style according to the tag.
    pub fn apply_tag(&self, tag: &BbcodeTag) -> Self {
        match tag.name() {
            "b" => Self {
                is_bold: true,
                ..self.clone()
            },
            "i" => Self {
                is_italic: true,
                ..self.clone()
            },
            _ => self.clone(),
        }
    }
}
