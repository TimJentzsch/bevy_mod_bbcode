use std::sync::Arc;

use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::bbcode::{parser::parse_bbcode, BbcodeNode, BbcodeTag};

use super::bbcode::{Bbcode, BbcodeSettings};

#[derive(Debug, Clone)]
struct BbcodeStyle {
    is_bold: bool,
    is_italic: bool,
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
    for node in nodes {
        match **node {
            BbcodeNode::Text(ref text) => {
                let font = match (style.is_bold, style.is_italic) {
                    (true, _) => settings.bold_font.clone(),
                    (_, true) => settings.italic_font.clone(),
                    (false, false) => settings.regular_font.clone(),
                };

                entity_commands.with_children(|builder| {
                    builder.spawn(TextBundle::from_section(
                        text.clone(),
                        TextStyle {
                            font,
                            // Allow font size configuration
                            font_size: 10.,
                            // Allow color configuration
                            color: Color::WHITE,
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
