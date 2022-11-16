use crate::prelude::*;
use bevy::{
    diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin},
    log::LogPlugin,
};

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LogPlugin {
            level: bevy::log::Level::INFO,
            filter: "wgpu_hal=warn,gfx_backend_metal=warn,wgpu_core=warn,bevy_render=info,lain=debug,bevy_render::render_resource::pipeline_cache=warn,bevy_app=debug".to_string(),
        });

        // Fps / Entity Tracking
        app.add_plugin(FrameTimeDiagnosticsPlugin).add_plugin(EntityCountDiagnosticsPlugin);

        // Inspector Egui Plugin
        app.add_plugin(DebugEguiPlugin);
    }
}
