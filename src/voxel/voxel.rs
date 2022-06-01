use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bracket_noise::prelude::*;
use std::collections::HashMap;

pub struct WorldData {
    pub chunks: HashMap<[i32; 2], Chunk>,
    pub noise_fn: FastNoise,
    pub player_chunk_pos: [i32; 2],
    pub world_settings: WorldSettings,
}

#[derive(Component, Reflect, Inspectable, Clone, Copy)]
#[reflect(Component)]
pub struct WorldSettings {
    pub noise_fractal_octaves: i32,
    pub noise_fractal_gain: f32,
    pub noise_fractal_lacunarity: f32,
    pub noise_frequency: f32,

    pub render_chunk_size: usize,
}

impl Default for WorldSettings {
    fn default() -> Self {
        Self {
            noise_fractal_octaves: 4,
            noise_fractal_gain: 2.,
            noise_fractal_lacunarity: 0.5,
            noise_frequency: 0.08,
            render_chunk_size: 4,
        }
    }
}
impl Default for WorldData {
    fn default() -> Self {
        Self {
            chunks: HashMap::new(),
            noise_fn: {
                let mut noise = FastNoise::seeded(10);
                noise.set_noise_type(NoiseType::PerlinFractal);
                noise.set_fractal_type(FractalType::FBM);
                noise.set_fractal_octaves(WorldSettings::default().noise_fractal_octaves);
                noise.set_fractal_gain(WorldSettings::default().noise_fractal_gain);
                noise.set_fractal_lacunarity(WorldSettings::default().noise_fractal_lacunarity);
                noise.set_frequency(WorldSettings::default().noise_frequency);
                noise
            },
            player_chunk_pos: [0, 0],
            world_settings: WorldSettings::default(),
        }
    }
}

impl WorldData {
    pub fn update_settings(&mut self, world_settings: &WorldSettings) {

        self.world_settings = *world_settings;

        let mut noise = FastNoise::seeded(10);
        noise.set_noise_type(NoiseType::PerlinFractal);
        noise.set_fractal_type(FractalType::FBM);
        noise.set_fractal_octaves(self.world_settings.noise_fractal_octaves);
        noise.set_fractal_gain(self.world_settings.noise_fractal_gain);
        noise.set_fractal_lacunarity(self.world_settings.noise_fractal_lacunarity);
        noise.set_frequency(self.world_settings.noise_frequency);
        self.noise_fn = noise;
    }

    pub fn frame_update_world(
        &mut self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) {
        let mut chunk_to_render: Vec<[i32; 2]> = Vec::new();

        for i in 0..self.world_settings.render_chunk_size + 1 {
            for t in 0..self.world_settings.render_chunk_size + 1 {
                let _i =
                    ((i as i32 - (self.world_settings.render_chunk_size as i32 / 2)) + self.player_chunk_pos[0]) as i32;
                let _t =
                    ((t as i32 - (self.world_settings.render_chunk_size as i32 / 2)) + self.player_chunk_pos[1]) as i32;
                chunk_to_render.push([_i, _t]);
            }
        }

        for chunk_position in chunk_to_render.iter() {
            if !self.chunks.contains_key(chunk_position) {
                let mut chunk = Chunk::spawn(*chunk_position);
                chunk.generate_data(&self.noise_fn);
                chunk.spawn_blocks(commands, meshes, materials);
                self.chunks.insert(chunk.position, chunk);
            }
        }

        let mut to_remove: Vec<[i32; 2]> = Vec::new();
        for (position, chunk) in &mut self.chunks {
            //Remove the entity
            //For the render generate entity
            if !chunk_to_render.contains(position) {
                if chunk.entity_id.id() != 0 {
                    commands.entity(chunk.entity_id).despawn();
                }
                to_remove.push(*position);
            }
        }

        for pos_to_remove in to_remove {
            //Clear the chunk that not render
            self.chunks.remove(&pos_to_remove);
        }
    }

    pub fn generate_data(&mut self) {
        for (_pos, chunk) in &mut self.chunks {
            chunk.generate_data(&self.noise_fn);
        }
    }
    pub fn spawn_blocks(
        &mut self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) {
        for (_pos, chunk) in &mut self.chunks {
            if chunk.entity_id.id() != 0 {
                commands.entity(chunk.entity_id).despawn();
            }
            chunk.spawn_blocks(commands, meshes, materials);
        }
    }

    pub fn position_to_chunk(x: i32, z: i32) -> [i32; 2] {
        let mut _x = x;
        let mut _z = z;
        if x < 0 {
            _x -= CHUNK_SIZE as i32;
        }
        if z < 0 {
            _z -= CHUNK_SIZE as i32;
        }
        [_x / CHUNK_SIZE as i32, _z / CHUNK_SIZE as i32]
    }
}

const DIRECTIONS: [[i8; 3]; 6] = [
    [0, 0, 1],
    [0, 0, -1],
    [1, 0, 0],
    [-1, 0, 0],
    [0, 1, 0],
    [0, -1, 0],
];

const VERTICES: [[([f32; 3], [f32; 3], [f32; 2]); 4]; 6] = [
    // Top
    [
        ([0., 0., 1.0], [0., 0., 1.0], [0., 0.]),
        ([1.0, 0., 1.0], [0., 0., 1.0], [1.0, 0.]),
        ([1.0, 1.0, 1.0], [0., 0., 1.0], [1.0, 1.0]),
        ([0., 1.0, 1.0], [0., 0., 1.0], [0., 1.0]),
    ],
    // Bottom
    [
        ([0., 1.0, 0.], [0., 0., -1.0], [1.0, 0.]),
        ([1.0, 1.0, 0.], [0., 0., -1.0], [0., 0.]),
        ([1.0, 0., 0.], [0., 0., -1.0], [0., 1.0]),
        ([0., 0., 0.], [0., 0., -1.0], [1.0, 1.0]),
    ],
    // Right
    [
        ([1.0, 0., 0.], [1.0, 0., 0.], [0., 0.]),
        ([1.0, 1.0, 0.], [1.0, 0., 0.], [1.0, 0.]),
        ([1.0, 1.0, 1.0], [1.0, 0., 0.], [1.0, 1.0]),
        ([1.0, 0., 1.0], [1.0, 0., 0.], [0., 1.0]),
    ],
    // Left
    [
        ([0., 0., 1.0], [-1.0, 0., 0.], [1.0, 0.]),
        ([0., 1.0, 1.0], [-1.0, 0., 0.], [0., 0.]),
        ([0., 1.0, 0.], [-1.0, 0., 0.], [0., 1.0]),
        ([0., 0., 0.], [-1.0, 0., 0.], [1.0, 1.0]),
    ],
    // Front
    [
        ([1.0, 1.0, 0.], [0., 1.0, 0.], [1.0, 0.]),
        ([0., 1.0, 0.], [0., 1.0, 0.], [0., 0.]),
        ([0., 1.0, 1.0], [0., 1.0, 0.], [0., 1.0]),
        ([1.0, 1.0, 1.0], [0., 1.0, 0.], [1.0, 1.0]),
    ],
    // Back
    [
        ([1.0, 0., 1.0], [0., -1.0, 0.], [0., 0.]),
        ([0., 0., 1.0], [0., -1.0, 0.], [1.0, 0.]),
        ([0., 0., 0.], [0., -1.0, 0.], [1.0, 1.0]),
        ([1.0, 0., 0.], [0., -1.0, 0.], [0., 1.0]),
    ],
];

const FACE_VERTICES_POS: [u32; 6] = [0, 1, 2, 2, 3, 0];

#[derive(Copy, Clone, Debug)]
pub enum BlockType {
    Air,
    Dirt,
}

#[derive(Copy, Clone, Debug)]
struct Block {
    block_type: BlockType,
}
pub const CHUNK_SIZE: usize = 32;
pub const CHUNK_HEIGHT: usize = 256;
const HALF_CHUNK_HEIGHT: usize = CHUNK_HEIGHT / 2;

#[derive(Debug, Clone)]
pub struct Chunk {
    entity_id: Entity,
    ///Position of the Chunk in the world, x, z
    position: [i32; 2],
    /// Array of 3D containing the data, also contains plus two size to calculate the mesh
    data: Vec<Vec<Vec<Block>>>,
}

impl Default for Chunk {
    fn default() -> Self {
        Self {
            entity_id: Entity::from_raw(0),
            position: [0i32; 2],
            data: vec![
                vec![
                    vec![
                        Block {
                            block_type: BlockType::Air,
                        };
                        CHUNK_SIZE + 2
                    ];
                    CHUNK_HEIGHT + 2
                ];
                CHUNK_SIZE + 2
            ],
        }
    }
}

impl Chunk {
    pub fn spawn(position: [i32; 2]) -> Chunk {
        Chunk {
            position: position.clone(),
            ..default()
        }
    }

    /// Return the [`BlockType`] of the position in the chunk
    ///
    /// Can got from -1 to [`CHUNK_SIZE`]+1
    pub fn get_block(&self, x: i32, y: i32, z: i32) -> BlockType {
        self.data[(x + 1) as usize][(y + 1) as usize][(z + 1) as usize].block_type
    }
    /// Set the [`BlockType`] of the position in the chunk
    ///
    /// Can set from -1 to [`CHUNK_SIZE`]+1
    pub fn set_block(&mut self, x: i32, y: i32, z: i32, block_type: BlockType) {
        self.data[(x + 1) as usize][(y + 1) as usize][(z + 1) as usize].block_type = block_type;
    }

    /// Return a Vec with the faces without neighbors of the block in x, y, z
    pub fn get_no_neighbors(&self, x: i32, y: i32, z: i32) -> Vec<u8> {
        let mut neighbors = Vec::new();
        let mut block_pos = [0, 0, 0];

        for i in 0..DIRECTIONS.len() {
            let dir = DIRECTIONS[i];
            block_pos[0] = x + dir[0] as i32;
            block_pos[1] = y + dir[1] as i32;
            block_pos[2] = z + dir[2] as i32;
            if let BlockType::Air = self.get_block(block_pos[0], block_pos[1], block_pos[2]) {
                neighbors.push(i as u8)
            }
        }
        neighbors
    }

    pub fn generate_data(&mut self, noise: &FastNoise) {
        self.data = vec![
            vec![
                vec![
                    Block {
                        block_type: BlockType::Air,
                    };
                    CHUNK_SIZE + 2
                ];
                CHUNK_HEIGHT + 2
            ];
            CHUNK_SIZE + 2
        ];
        let mut block_pos = [0.0f32, 0.0f32, 0.0f32];
        for _x in 0..CHUNK_SIZE + 2 {
            for _z in 0..CHUNK_SIZE + 2 {
                let x = _x as i32 - 1;
                let z = _z as i32 - 1;
                block_pos[0] = 0.1 + ((self.position[0] * CHUNK_SIZE as i32) + x) as f32;
                block_pos[2] = 0.1 + ((self.position[1] * CHUNK_SIZE as i32) + z) as f32;

                let height = (noise.get_noise(block_pos[0], block_pos[2])
                    * HALF_CHUNK_HEIGHT as f32) as i32
                    + HALF_CHUNK_HEIGHT as i32;
                for y in 0..height.clamp(-1,(CHUNK_HEIGHT+1) as i32) {
                    self.set_block(x, y, z, BlockType::Dirt);
                }
            }
        }
    }

    pub fn spawn_blocks(
        &mut self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) {
        let mut block_pos = [0.0f32, 0.0f32, 0.0f32];

        let mut vertices = Vec::new();
        let mut indices_vec: Vec<u32> = Vec::new();

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_HEIGHT {
                for z in 0..CHUNK_SIZE {
                    block_pos[0] = (self.position[0] + x as i32) as f32;
                    block_pos[1] = y as f32;
                    block_pos[2] = (self.position[1] + z as i32) as f32;
                    if let BlockType::Air = self.get_block(x as i32, y as i32, z as i32) {
                        continue;
                    } else {
                        // Calcule the faces
                        let neighbors = self.get_no_neighbors(x as i32, y as i32, z as i32);

                        for neighbor in neighbors {
                            let mut new_face = VERTICES[neighbor as usize].clone();
                            for i in 0..4 {
                                for j in 0..3 {
                                    new_face[i].0[j] += block_pos[j];
                                }
                                vertices.push(new_face[i]);
                            }
                            let new_index = FACE_VERTICES_POS.clone();

                            indices_vec.append(
                                &mut new_index
                                    .map(|v| v + ((indices_vec.len() / 6) as u32 * 4))
                                    .to_vec(),
                            );
                        }
                    }
                }
            }
        }

        let indices = bevy::render::mesh::Indices::U32(indices_vec);
        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();

        for (position, normal, uv) in vertices.iter() {
            positions.push(*position);
            normals.push(*normal);
            uvs.push(*uv);
        }

        let mut mesh = Mesh::new(bevy::render::mesh::PrimitiveTopology::TriangleList);
        mesh.set_indices(Some(indices));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

        let material = materials.add(Color::rgb(165. / 255., 42. / 255., 42. / 255.).into());
        self.entity_id = commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(mesh),
                material: material,
                transform: Transform::from_xyz(
                    (self.position[0] * (CHUNK_SIZE - 1) as i32) as f32,
                    0.,
                    (self.position[1] * (CHUNK_SIZE - 1) as i32) as f32,
                ),
                ..default()
            })
            .id();
    }
}
