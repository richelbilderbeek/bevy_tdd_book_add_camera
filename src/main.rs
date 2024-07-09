use crate::app::*;
use bevy::prelude::*;
mod app;

fn main() {
    let initial_camera_scale = 0.2;
    let mut app = create_app(initial_camera_scale);
    app.add_plugins(DefaultPlugins);
    app.run();
}
