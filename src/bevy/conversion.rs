use std::sync::Arc;

use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::bbcode::{parser::parse_bbcode, BbcodeNode, BbcodeTag};

use super::bbcode::{Bbcode, BbcodeSettings};

#[derive(Debug, Clone)]
struct BbcodeStyle {
    is_bold: bool,
    is_italic: bool,
    color: Color,
}

impl BbcodeStyle {
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
                        Self {
                            color: color.into(),
                            ..self.clone()
                        }
                    } else {
                        warn!("Invalid bbcode color {color}");
                        self.clone()
                    }
                } else {
                    warn!("Missing bbcode color on [{}] tag", tag.name());
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
) {
    for (entity, bbcode, settings) in bbcode_query.iter() {
        if !bbcode.is_changed() && !settings.is_changed() {
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
            BbcodeStyle {
                is_bold: false,
                is_italic: false,
                color: settings.color,
            },
            &settings,
            &nodes,
        )
    }
}

fn construct_recursively(
    entity_commands: &mut EntityCommands,
    style: BbcodeStyle,
    settings: &BbcodeSettings,
    nodes: &Vec<Arc<BbcodeNode>>,
) {
    let default_font = settings.regular_font.clone().unwrap_or_default();

    for node in nodes {
        match **node {
            BbcodeNode::Text(ref text) => {
                let font = match (style.is_bold, style.is_italic) {
                    (true, _) => default_font.clone(),
                    (_, true) => settings
                        .italic_font
                        .clone()
                        .unwrap_or_else(|| default_font.clone()),
                    (false, false) => settings
                        .regular_font
                        .clone()
                        .unwrap_or_else(|| default_font.clone()),
                };

                entity_commands.with_children(|builder| {
                    builder.spawn(TextBundle::from_section(
                        text.clone(),
                        TextStyle {
                            font,
                            font_size: settings.font_size,
                            color: style.color,
                        },
                    ));
                });
            }

            BbcodeNode::Tag(ref tag) => construct_recursively(
                entity_commands,
                style.apply_tag(tag),
                settings,
                tag.children(),
            ),
        }
    }
}
