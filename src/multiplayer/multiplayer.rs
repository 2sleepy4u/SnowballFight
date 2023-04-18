pub use bevy::prelude::*;
pub use bevy_renet::renet::*;
pub use bevy_renet::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientMessage {
    Ping,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerMessage {
    Pong,
}

pub const PROTOCOL_ID: u64 = 1000;

use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
};


impl Material for CelMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/toon_material.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct CelMaterial {
    #[uniform(0)]
    pub color: Color,
    #[uniform(0)]
    pub light: Vec3,
    //pub lights: [Vec3; 10],
    pub alpha_mode: AlphaMode,
}
