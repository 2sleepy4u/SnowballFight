use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;


mod scene;
mod player;
mod collision;
pub use player::PlayerPlugin;
pub use scene::*;
pub use collision::*;
use snowball_fight::CelMaterial;

const MAX_WIDTH: f32 = 1280.0;
const MAX_HEIGHT: f32 = 720.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Test".into(),
                    resolution: (MAX_WIDTH, MAX_HEIGHT).into(),
                    resizable: false,
                    ..default()
                }),
                ..default()
            }),
        )
        .add_plugin(MaterialPlugin::<CelMaterial>::default())
        .add_plugin(CollisionPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(MapPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .run();
}

