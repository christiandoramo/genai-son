// Resolve dinamicamente para qual lado é "Para Baixo"
fn get_gravity_dir(x: u32, y: u32, z: u32) -> vec3<i32> {
    let rx = f32(x) - 128.0; let ry = f32(y) - 128.0; let rz = f32(z) - 128.0;
    let ax = abs(rx); let ay = abs(ry); let az = abs(rz);
    if (ax >= ay && ax >= az) { return vec3<i32>( -i32(sign(rx)), 0, 0 ); }
    if (ay >= ax && ay >= az) { return vec3<i32>( 0, -i32(sign(ry)), 0 ); }
    return vec3<i32>( 0, 0, -i32(sign(rz)) );
}

fn get_orthogonal(g: vec3<i32>, index: u32) -> vec3<i32> {
    var u = vec3<i32>(0); var v = vec3<i32>(0);
    if (g.x != 0) { u = vec3<i32>(0,1,0); v = vec3<i32>(0,0,1); }
    else if (g.y != 0) { u = vec3<i32>(1,0,0); v = vec3<i32>(0,0,1); }
    else { u = vec3<i32>(1,0,0); v = vec3<i32>(0,1,0); }
    if (index == 0u) { return u; } if (index == 1u) { return -u; } if (index == 2u) { return v; } return -v;
}

fn simulate_sand(idx: u32, x: u32, y: u32, z: u32, rand: u32) {
    let g = get_gravity_dir(x, y, z);
    let dx = u32(i32(x) + g.x); let dy = u32(i32(y) + g.y); let dz = u32(i32(z) + g.z);
    
    if (!is_valid(dx, dy, dz)) { return; }
    let down = get_index(dx, dy, dz);
    
    // Queda livre baseada em G!
    if (world.data[down] == 0u) { move_voxel(idx, down, dx, dy, dz, 1u); return; }

    let side = get_orthogonal(g, rand % 4u);
    let d1x = u32(i32(dx) + side.x); let d1y = u32(i32(dy) + side.y); let d1z = u32(i32(dz) + side.z);
    
    if (is_valid(d1x, d1y, d1z)) {
        let diag1 = get_index(d1x, d1y, d1z);
        if (world.data[diag1] == 0u) { move_voxel(idx, diag1, d1x, d1y, d1z, 1u); return; }
    }
}

fn simulate_liquid(idx: u32, x: u32, y: u32, z: u32, rand: u32, mat_id: u32, viscosity: u32) {
    let g = get_gravity_dir(x, y, z);
    let dx = u32(i32(x) + g.x); let dy = u32(i32(y) + g.y); let dz = u32(i32(z) + g.z);
    
    if (!is_valid(dx, dy, dz)) { return; }
    let down = get_index(dx, dy, dz);
    
    if (world.data[down] == 0u) { move_voxel(idx, down, dx, dy, dz, mat_id); return; }
    if (rand % viscosity != 0u) { return; }

    let side = get_orthogonal(g, rand % 4u);
    let sx = u32(i32(x) + side.x); let sy = u32(i32(y) + side.y); let sz = u32(i32(z) + side.z);
    
    if (is_valid(sx, sy, sz)) {
        let s_idx = get_index(sx, sy, sz);
        if (world.data[s_idx] == 0u) { 
            // Checa cachoeira na direção G!
            let sdx = u32(i32(sx) + g.x); let sdy = u32(i32(sy) + g.y); let sdz = u32(i32(sz) + g.z);
            if (is_valid(sdx, sdy, sdz)) {
                let side_down = get_index(sdx, sdy, sdz);
                if (world.data[side_down] == 0u) {
                    move_voxel(idx, s_idx, sx, sy, sz, mat_id); return;
                }
            }
            move_voxel(idx, s_idx, sx, sy, sz, mat_id); return; 
        }
    }
}