use crate::biomes::generate_voxel;
use noise::OpenSimplex;

pub const WORLD_X: usize = 512;
pub const WORLD_Y: usize = 128;
pub const WORLD_Z: usize = 512;

pub struct World {
    pub data: Vec<u32>,
    pub macro_data: Vec<u32>,
}

impl World {
    pub fn generate_new() -> Self {
        let mut data = vec![0u32; WORLD_X * WORLD_Y * WORLD_Z];
        let mut macro_data = vec![0u32; (WORLD_X / 8) * (WORLD_Y / 8) * (WORLD_Z / 8)];
        let noise_gen = OpenSimplex::new(1998); // Seed do mapa
        
        println!("Gerando Biomas e Cavernas 3D...");
        for x in 0..WORLD_X {
            for z in 0..WORLD_Z {
                for y in 0..WORLD_Y {
                    let voxel = generate_voxel(x, y, z, &noise_gen);
                    if voxel != 0 {
                        data[x + y * WORLD_X + z * WORLD_X * WORLD_Y] = voxel;
                        let macro_index = (x / 8) + (y / 8) * (WORLD_X / 8) + (z / 8) * (WORLD_X / 8) * (WORLD_Y / 8);
                        macro_data[macro_index] = 1;
                    }
                }
            }
        }
        Self { data, macro_data }
    }
}