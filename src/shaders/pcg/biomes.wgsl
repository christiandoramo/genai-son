#define_import_path biomes

fn get_biome_color(mat_id: u32, dist: f32) -> vec3<f32> {
    switch(mat_id) {
        case 1u: { return vec3<f32>(0.9, 0.8, 0.2); } // Areia
        case 2u: { return vec3<f32>(0.1, 0.4, 0.8); } // Água
        case 4u: { return vec3<f32>(0.9, 0.95, 1.0); } // Neve
        case 5u: { return vec3<f32>(0.4, 0.25, 0.1); } // Terra
        case 7u: { return vec3<f32>(0.2, 0.6, 0.2); } // Grama
        case 8u: { return vec3<f32>(0.4, 0.4, 0.4); } // Pedra
        case 9u: { return vec3<f32>(1.0, 0.3, 0.0); } // Magma
        case 10u: { return vec3<f32>(0.8, 0.85, 0.9); } // Ferro
        default: { return vec3<f32>(0.5); }
    }
}