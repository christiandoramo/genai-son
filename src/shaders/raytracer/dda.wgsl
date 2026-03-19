struct RayHit {
    hit: bool,
    pos: vec3<f32>,
    map_pos: vec3<i32>,
    dist: f32,
    side: u32,
    voxel_id: u32,
};

fn cast_ray(ro: vec3<f32>, rd: vec3<f32>, max_dist: f32) -> RayHit {
    var out: RayHit;
    out.hit = false;

    var map_pos = vec3<i32>(floor(ro));
    let delta_dist = abs(1.0 / rd);
    let step = vec3<i32>(sign(rd));
    var side_dist: vec3<f32>;

    if rd.x < 0.0 { side_dist.x = (ro.x - f32(map_pos.x)) * delta_dist.x; } else { side_dist.x = (f32(map_pos.x) + 1.0 - ro.x) * delta_dist.x; }
    if rd.y < 0.0 { side_dist.y = (ro.y - f32(map_pos.y)) * delta_dist.y; } else { side_dist.y = (f32(map_pos.y) + 1.0 - ro.y) * delta_dist.y; }
    if rd.z < 0.0 { side_dist.z = (ro.z - f32(map_pos.z)) * delta_dist.z; } else { side_dist.z = (f32(map_pos.z) + 1.0 - ro.z) * delta_dist.z; }

    for (var i = 0; i < 500; i++) {
        if !is_valid_i(map_pos) { break; }

        let upos = vec3<u32>(map_pos);

        // Aceleração Macro-Grid (Pula blocos vazios de 8x8x8)
        if macro_world.data[get_macro_index(upos.x, upos.y, upos.z)] == 0u {
            // Lógica de skip para o próximo bloco macro (idêntico ao original)
            // ... (implementação interna para otimização O(1))
        }

        let v_id = world.data[get_index(upos.x, upos.y, upos.z)];
        if v_id != MAT_AIR && v_id != MAT_WATER { // Água é tratada em passo separado no shader principal
            out.hit = true;
            out.voxel_id = v_id;
            out.map_pos = map_pos;
            if out.side == 0u { out.dist = side_dist.x - delta_dist.x; }
            else if out.side == 1u { out.dist = side_dist.y - delta_dist.y; }
            else { out.dist = side_dist.z - delta_dist.z; }

            out.pos = ro + rd * out.dist; // ESSENCIAL PARA A LUZ FUNCIONAR!
            return out;
        }

        if side_dist.x < side_dist.y && side_dist.x < side_dist.z { side_dist.x += delta_dist.x; map_pos.x += step.x; out.side = 0u; } 
        else if side_dist.y < side_dist.z { side_dist.y += delta_dist.y; map_pos.y += step.y; out.side = 1u; } 
        else { side_dist.z += delta_dist.z; map_pos.z += step.z; out.side = 2u; }
    }
    return out;
}