use bevy::prelude::*;
//use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_editor_pls::prelude::*;

mod scene;
mod player;
mod collision;
mod snowball;
mod loadout;

pub use player::PlayerPlugin;
pub use scene::*;
pub use collision::*;
use snowball_fight::materials::CelMaterial;
pub use snowball::*;
use loadout::components::{AttackLoadoutList, DashLoadoutList};
use loadout::systems::load_loadout;

const MAX_WIDTH: f32 = 1280.0;
const MAX_HEIGHT: f32 = 720.0;

fn main() {
    App::new()
        .insert_resource(AttackLoadoutList { list: Vec::new() })
        .insert_resource(DashLoadoutList { list: Vec::new() })
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
            },
            )
        )
        .add_plugin(EditorPlugin::default())
        .add_startup_system(load_loadout)
        .add_plugin(MaterialPlugin::<CelMaterial>::default())
        .add_plugin(CollisionPlugin)
        .add_plugin(SnowballPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(MapPlugin)
        //.add_plugin(WorldInspectorPlugin::new())
        .run();
}

