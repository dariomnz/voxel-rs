use super::voxel;
use bevy::prelude::*;
use bevy_flycam::*;

pub struct VoxelWorldPlugin;

impl Plugin for VoxelWorldPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<voxel::WorldSettings>()
            .init_resource::<voxel::WorldData>()
            .add_startup_system(setup_voxel)
            .add_system(update_noise)
            .add_system(frame_world);
    }
}

fn setup_voxel(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut world_data: ResMut<voxel::WorldData>,
) {
    commands.spawn().insert(voxel::WorldSettings::default());
    world_data.frame_update_world(
        &mut commands,
        &mut meshes,
        &mut materials,
    );
}

fn update_noise(
    mut world_data: ResMut<voxel::WorldData>,
    world_settings_query: Query<&voxel::WorldSettings, (With<voxel::WorldSettings>, Changed<voxel::WorldSettings>)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if let Ok(world_settings) = world_settings_query.get_single() {
        world_data.update_settings(world_settings);
        world_data.generate_data();
        world_data.spawn_blocks(
            &mut commands,
            &mut meshes,
            &mut materials,
        );
    }
}

fn frame_world(
    query: Query<&Transform, With<FlyCam>>,
    mut world_data: ResMut<voxel::WorldData>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let transform = query.single();
    world_data.player_chunk_pos = voxel::WorldData::position_to_chunk(
        transform.translation.x as i32,
        transform.translation.z as i32,
    );

    world_data.frame_update_world(
        &mut commands,
        &mut meshes,
        &mut materials,
    );
}
