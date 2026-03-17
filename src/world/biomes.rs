use noise::{NoiseFn, OpenSimplex};

pub fn generate_voxel(x: usize, y: usize, z: usize, noise: &OpenSimplex) -> u32 {
    if y == 0 { return 4; } // Bedrock Indestrutível no fundo absoluto

    let px = x as f64 * 0.01;
    let py = y as f64 * 0.01;
    let pz = z as f64 * 0.01;

    // Relevo Base: Montanhas suaves e vales
    let ground = noise.get([px, pz]) * 20.0;
    let detail = noise.get([px * 3.0, pz * 3.0]) * 5.0;
    let height = (ground + detail + 40.0).max(1.0) as usize;

    let sea_level = 30;

    // CAVERNAS VOLUMÉTRICAS ("Worms")
    // Usamos o valor absoluto do ruído para criar tubos finos (cavernas) que cortam o cenário
    let cave_noise = noise.get([px * 2.5, py * 2.5, pz * 2.5]).abs();
    let is_cave = cave_noise < 0.08 && y < height; 

    // Acima do chão
    if y > height {
        if y <= sea_level { return 2; } // Água pré-pronta nos vales baixos
        return 0; // Ar
    }

    // Se estivermos dentro da área de uma caverna
    if is_cave {
        if y < 8 { return 9; } // Magma no fundo das cavernas profundas!
        return 0; // Ar da caverna
    }

    // Pintura dos Materiais
    if y == height && y > sea_level { return 7; } // Grama no topo
    if y > height.saturating_sub(4) { return 5; } // Terra abaixo da grama
    
    return 8; // Pedra maciça no miolo
}