use std::sync::Arc;

use bevy::{ecs::system::EntityCommands, prelude::*, ui::FocusPolicy, utils::HashMap};

#[derive(Debug, Clone, Component, Default)]

pub struct Bbcode {
    /// The bbcode-formatted text.
    pub content: String,
}

type ModifierFn = dyn Fn(&mut EntityCommands) + Send + Sync;

#[derive(Clone, Default)]
pub(crate) struct Modifiers {
    pub(crate) modifier_map: HashMap<String, Arc<ModifierFn>>,
}

#[derive(Clone, Component)]
pub struct BbcodeSettings {
    pub font_size: f32,
    pub color: Color,

    pub(crate) regular_font: Option<Handle<Font>>,
    pub(crate) bold_font: Option<Handle<Font>>,
    pub(crate) italic_font: Option<Handle<Font>>,

    pub(crate) modifiers: Modifiers,
}

impl BbcodeSettings {
    pub fn new(font_size: f32, color: Color) -> Self {
        Self {
            font_size,
            color,
            regular_font: None,
            bold_font: None,
            italic_font: None,
            modifiers: Default::default(),
        }
    }

    /// Add a font to use for regular text.
    pub fn with_regular_font(mut self, handle: Handle<Font>) -> Self {
        self.regular_font = Some(handle);
        self
    }

    /// Add a font to use for bold text.
    pub fn with_bold_font(mut self, handle: Handle<Font>) -> Self {
        self.bold_font = Some(handle);
        self
    }

    /// Add a font to use for italic text.
    pub fn with_italic_font(mut self, handle: Handle<Font>) -> Self {
        self.italic_font = Some(handle);
        self
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

#[derive(Bundle)]
pub struct BbcodeBundle {
    pub bbcode: Bbcode,
    pub bbcode_settings: BbcodeSettings,

    // --- NODE BUNDLE ---
    /// Describes the logical size of the node
    pub node: Node,
    /// Styles which control the layout (size and position) of the node and its children
    /// In some cases these styles also affect how the node drawn/painted.
    pub style: Style,
    /// The background color, which serves as a "fill" for this node
    pub background_color: BackgroundColor,
    /// The color of the Node's border
    pub border_color: BorderColor,
    /// The border radius of the node
    pub border_radius: BorderRadius,
    /// Whether this node should block interaction with lower nodes
    pub focus_policy: FocusPolicy,
    /// The transform of the node
    ///
    /// This component is automatically managed by the UI layout system.
    /// To alter the position of the `NodeBundle`, use the properties of the [`Style`] component.
    pub transform: Transform,
    /// The global transform of the node
    ///
    /// This component is automatically updated by the `TransformPropagate` systems.
    /// To alter the position of the `NodeBundle`, use the properties of the [`Style`] component.
    pub global_transform: GlobalTransform,
    /// Describes the visibility properties of the node
    pub visibility: Visibility,
    /// Inherited visibility of an entity.
    pub inherited_visibility: InheritedVisibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub view_visibility: ViewVisibility,
    /// Indicates the depth at which the node should appear in the UI
    pub z_index: ZIndex,
}

impl BbcodeBundle {
    pub fn from_content<S: Into<String>>(content: S, settings: BbcodeSettings) -> Self {
        Self {
            bbcode: Bbcode {
                content: content.into(),
            },
            bbcode_settings: settings,

            background_color: default(),
            border_color: default(),
            border_radius: default(),
            focus_policy: default(),
            global_transform: default(),
            inherited_visibility: default(),
            node: default(),
            style: default(),
            transform: default(),
            view_visibility: default(),
            visibility: default(),
            z_index: default(),
        }
    }
}
