use bevy::prelude::*;
use bevy::text::Font;

#[derive(Debug, Clone, Component)]
pub struct BbcodeSettings {
    pub normal_font: Handle<Font>,
    pub bold_font: Handle<Font>,
    pub italic_font: Handle<Font>,
}
