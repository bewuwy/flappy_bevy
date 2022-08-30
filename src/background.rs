use bevy::prelude::*;

mod clouds;
mod hills;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(clouds::CloudsPlugin)
            .add_plugin(hills::HillsPlugin);
    }
}
