mod gui;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(gui::GuiPlugin)
        .run();
}
