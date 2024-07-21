use bevy::prelude::*;

use super::conversion::convert_bbcode;

pub struct BbcodePlugin;

impl Plugin for BbcodePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, convert_bbcode);
    }
}
