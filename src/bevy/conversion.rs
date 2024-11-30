use std::sync::Arc;

use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::bbcode::{parser::parse_bbcode, BbcodeNode, BbcodeTag};

use super::{
    bbcode::{Bbcode, BbcodeSettings},
    color::{BbCodeColor, BbCodeColored},
    font::FontRegistry,
    ColorMap,
};

#[derive(Debug, Clone)]
struct BbcodeContext {
    /// The name of the font family to use for the text.
    font_family: String,
    /// Whether the text should be written **bold**.
    is_bold: bool,
    /// Whether the text should be written *italic*.
    is_italic: bool,
    /// The color of the text.
    color: BbCodeColor,

    /// Marker components to apply to the spawned `Text`s.
    markers: Vec<String>,
}

impl BbcodeContext {
    /// Change the style according to the tag.
    fn apply_tag(&self, tag: &BbcodeTag) -> Self {
        match tag.name() {
            "b" => Self {
                is_bold: true,
                ..self.clone()
            },
            "i" => Self {
                is_italic: true,
                ..self.clone()
            },
            "c" | "color" => {
                if let Some(color) = tag.simple_param() {
                    if let Ok(color) = Srgba::hex(color.trim()) {
                        let color: Color = color.into();
                        Self {
                            color: color.into(),
                            ..self.clone()
                        }
                    } else {
                        Self {
                            color: color.to_string().into(),
                            ..self.clone()
                        }
                    }
                } else {
                    warn!("Missing bbcode color on [{}] tag", tag.name());
                    self.clone()
                }
            }
            "m" | "marker" => {
                if let Some(marker) = tag.simple_param() {
                    let mut markers = self.markers.clone();
                    markers.push(marker.to_string());

                    Self {
                        markers,
                        ..self.clone()
                    }
                } else {
                    warn!("Missing marker name on [{}] tag", tag.name());
                    self.clone()
                }
            }
            "font" => {
                if let Some(font_family) = tag.simple_param() {
                    Self {
                        font_family: font_family.to_string(),
                        ..self.clone()
                    }
                } else {
                    warn!("Missing font family name on [{}] tag", tag.name());
                    self.clone()
                }
            }
            _ => self.clone(),
        }
    }
}

pub fn convert_bbcode(
    mut commands: Commands,
    bbcode_query: Query<(Entity, Ref<Bbcode>, Ref<BbcodeSettings>)>,
    font_registry: Res<FontRegistry>,
    color_map: Res<ColorMap>,
) {
    for (entity, bbcode, settings) in bbcode_query.iter() {
        if !bbcode.is_changed() && !settings.is_changed() && !font_registry.is_changed() {
            continue;
        }

        let mut entity_commands = commands.entity(entity);

        // Clear out all descendants to start fresh
        entity_commands.despawn_descendants();

        let Ok((remainder, nodes)) = parse_bbcode(&bbcode.content) else {
            warn!("Failed to parse bbcode\n{}", bbcode.content);
            continue;
        };

        if !remainder.is_empty() {
            warn!("Failed to parse bbcode\n{}", bbcode.content);
            continue;
        }

        construct_recursively(
            &mut entity_commands,
            BbcodeContext {
                font_family: settings.font_family.clone(),
                is_bold: false,
                is_italic: false,
                color: settings.color.clone(),
                markers: Vec::new(),
            },
            &settings,
            &nodes,
            font_registry.as_ref(),
            color_map.as_ref(),
        )
    }
}

fn construct_recursively(
    entity_commands: &mut EntityCommands,
    context: BbcodeContext,
    settings: &BbcodeSettings,
    nodes: &Vec<Arc<BbcodeNode>>,
    font_registry: &FontRegistry,
    color_map: &ColorMap,
) {
    for node in nodes {
        match **node {
            BbcodeNode::Text(ref text) => {
                let font_query = fontdb::Query {
                    families: &[
                        fontdb::Family::Name(&context.font_family),
                        fontdb::Family::Name(&settings.font_family),
                    ],
                    weight: if context.is_bold {
                        fontdb::Weight::BOLD
                    } else {
                        fontdb::Weight::NORMAL
                    },
                    stretch: fontdb::Stretch::Normal,
                    style: if context.is_italic {
                        fontdb::Style::Italic
                    } else {
                        fontdb::Style::Normal
                    },
                };
                let font = font_registry.query_handle(&font_query).unwrap_or_default();

                entity_commands.with_children(|builder| {
                    let mut text_commands = builder.spawn((
                        TextSpan::new(text.clone()),
                        TextFont {
                            font,
                            font_size: settings.font_size,
                            ..default()
                        },
                        TextColor(context.color.to_color(color_map).unwrap_or(Color::WHITE)),
                    ));

                    // Track named colors for efficient update
                    if let BbCodeColor::Named(name) = &context.color {
                        text_commands.insert(BbCodeColored { name: name.clone() });
                    }

                    // Apply marker components
                    for marker in &context.markers {
                        if let Some(modifier) = settings.modifiers.modifier_map.get(marker) {
                            modifier(&mut text_commands);
                        }
                    }
                });
            }

            BbcodeNode::Tag(ref tag) => construct_recursively(
                entity_commands,
                context.apply_tag(tag),
                settings,
                tag.children(),
                font_registry,
                color_map,
            ),
        }
    }
}
