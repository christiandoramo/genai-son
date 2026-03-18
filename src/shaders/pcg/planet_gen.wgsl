@group(0) @binding(0) var<uniform> uniforms: Uniforms;
@group(0) @binding(1) var<storage, read_write> world: WorldBuffer;
@group(0) @binding(2) var<storage, read_write> macro_world: WorldBuffer;

@compute @workgroup_size(4, 4, 4)
fn main_gen(@builtin(global_invocation_id) id: vec3<u32>) {
    if (id.x >= WORLD_SIZE || id.y >= WORLD_SIZE || id.z >= WORLD_SIZE) { return; }

    let pos = vec3<f32>(id) - vec3<f32>(128.0);
    
    // Matemática do Superelipsoide (Cúbica)
    let dist_base = sqrt(sqrt(dot(pos*pos, pos*pos)));

    if (dist_base > 80.0) { return; }

    let dir = normalize(pos);
    
    // Delegar para biomas (Lógica unificada)
    let material = calculate_biome_material(dist_base, dir, pos);

    let idx = get_index(id.x, id.y, id.z);
    world.data[idx] = material;
    
    if (material != MAT_AIR) {
        macro_world.data[get_macro_index(id.x, id.y, id.z)] = 1u;
    }
}

fn calculate_biome_material(dist: f32, dir: vec3<f32>, local_pos: vec3<f32>) -> u32 {
    let cont = noise(dir * 1.2);
    let colinas = max(noise(dir * 3.0), 0.0);
    
    var h = 40.0 + (cont * 10.0);
    if (cont > -0.1) {
        h += colinas * 12.0;
    }

    let superficie = round(h / 2.0) * 2.0;
    
    if (dist <= superficie) {
        let profundidade = superficie - dist;
        
        // Regra de Cavernas
        let cave = abs(noise(local_pos * 0.08));
        if (cave < 0.05 && dist > 25.0) { return MAT_AIR; }

        // Estratificação Geológica
        if (dist < 25.0) { return MAT_MAGMA; }
        if (dist < 20.0) { return MAT_IRON; }
        
        if (profundidade < 1.0) {
            if (h > 55.0) { return MAT_SNOW; }
            if (cont < -0.2) { return MAT_SAND; }
            return MAT_GRASS;
        }
        return MAT_ROCK;
    }
    
    if (dist <= 40.0) { return MAT_WATER; }
    
    return MAT_AIR;
}