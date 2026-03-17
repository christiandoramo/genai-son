fn simulate_sand(idx: u32, x: u32, y: u32, z: u32, rand: u32) {
    let down = get_index(x, y - 1u, z);
    if (world.data[down] == 0u) { move_voxel(idx, down, x, y - 1u, z, 1u); return; }
    let dx = select(x - 1u, x + 1u, (rand % 2u) == 0u); let dz = select(z - 1u, z + 1u, (rand / 2u) == 0u);
    let diag1 = get_index(dx, y - 1u, z); if (world.data[diag1] == 0u) { move_voxel(idx, diag1, dx, y - 1u, z, 1u); return; }
    let diag2 = get_index(x, y - 1u, dz); if (world.data[diag2] == 0u) { move_voxel(idx, diag2, x, y - 1u, dz, 1u); return; }
}

fn simulate_liquid(idx: u32, x: u32, y: u32, z: u32, rand: u32, mat_id: u32, viscosity: u32) {
    let down = get_index(x, y - 1u, z);
    if (world.data[down] == 0u) { move_voxel(idx, down, x, y - 1u, z, mat_id); return; }
    
    // Viscosidade: Se for magma (lento), ele só processa o espalhamento horizontal a cada X frames
    if (rand % viscosity != 0u) { return; }

    let dirs = array<vec2<u32>, 4>(vec2<u32>(1u, 0u), vec2<u32>(0u, 1u), vec2<u32>(4294967295u, 0u), vec2<u32>(0u, 4294967295u));
    
    // 1. Procura buracos profundos (Cachoeira)
    for(var i = 0u; i < 4u; i++) {
        let d = dirs[(rand + i) % 4u]; 
        let side = get_index(x + d.x, y, z + d.y);
        if (world.data[side] == 0u) {
            let side_down = get_index(x + d.x, y - 1u, z + d.y);
            if (world.data[side_down] == 0u) { move_voxel(idx, side, x + d.x, y, z + d.y, mat_id); return; }
        }
    }
    
    // 2. Espalhamento rasteiro (Preenche os lagos)
    // Para líquidos espalharem rápido, eles checam as laterais aleatórias de maneira agressiva.
    for(var i = 0u; i < 4u; i++) {
        let d = dirs[(rand + i + 2u) % 4u]; 
        let side = get_index(x + d.x, y, z + d.y);
        if (world.data[side] == 0u) {
            move_voxel(idx, side, x + d.x, y, z + d.y, mat_id); return;
        }
    }
}