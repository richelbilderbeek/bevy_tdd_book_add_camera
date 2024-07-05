use crate::app::*;
use crate::game_parameters::*;
use bevy::prelude::*;
mod app;
mod game_parameters;

fn main() {
    let mut app = create_app(create_default_game_parameters());
    app.add_plugins(DefaultPlugins);
    app.run();
}
