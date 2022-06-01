use std::env;

use bevy::prelude::*;
use bevy_flycam::*;
use bevy_inspector_egui::WorldInspectorPlugin;
mod voxel;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system_to_stage(StartupStage::PostStartup, setup)
        .add_plugin(bevy_screen_diags::ScreenDiagsPlugin)
        .add_plugin(voxel::plugin::VoxelWorldPlugin)
        .run();
}

fn setup(mut commands: Commands, mut player_query: Query<&mut Transform, With<FlyCam>>) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        player_transform.translation.y = 200.;
    }
    // directional 'sun' light
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            // shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 256.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..default()
        },
        ..default()
    });

    commands.spawn_bundle(UiCameraBundle::default());
}
