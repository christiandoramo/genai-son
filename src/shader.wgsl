struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

struct Uniforms {
    resolution: vec2<f32>,
    time: f32,
    _padding1: f32,
    camera_pos: vec3<f32>,
    _padding2: f32,
    camera_front: vec3<f32>,
    _padding3: f32,
};

struct WorldBuffer {
    data: array<u32>,
};

@group(0) @binding(0) var<uniform> uniforms: Uniforms;
@group(0) @binding(1) var<storage, read> world: WorldBuffer;

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    let x = f32((in_vertex_index << 1u) & 2u);
    let y = f32(in_vertex_index & 2u);
    out.clip_position = vec4<f32>(x * 2.0 - 1.0, 1.0 - y * 2.0, 0.0, 1.0);
    out.uv = vec2<f32>(x, y);
    return out;
}

// Limites globais do nosso "Mundo" atual
const WORLD_SIZE = vec3<f32>(512.0, 128.0, 512.0);

fn get_voxel(p: vec3<i32>) -> u32 {
    if (p.x < 0 || p.x >= 512 || p.y < 0 || p.y >= 128 || p.z < 0 || p.z >= 512) {
        return 0u; 
    }
    let index = u32(p.x) + u32(p.y) * 512u + u32(p.z) * 512u * 128u;
    return world.data[index];
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var uv = in.clip_position.xy / uniforms.resolution.xy;
    uv = uv * 2.0 - 1.0;
    uv.x *= uniforms.resolution.x / uniforms.resolution.y;
    uv.y = -uv.y;

    var ro = uniforms.camera_pos; 
    let forward = normalize(uniforms.camera_front);
    let right = normalize(cross(vec3<f32>(0.0, 1.0, 0.0), forward));
    let up = cross(forward, right);
    let rd = normalize(forward + uv.x * right + uv.y * up);

    // --- OTIMIZAÇÃO AABB (Ray-Box Intersection) ---
    // Checa se o raio intercepta a área 512x128x512 onde a memória existe.
    let tmin = (vec3<f32>(0.0) - ro) / rd;
    let tmax = (WORLD_SIZE - ro) / rd;
    let t1 = min(tmin, tmax);
    let t2 = max(tmin, tmax);
    let t_near = max(max(t1.x, t1.y), t1.z);
    let t_far = min(min(t2.x, t2.y), t2.z);
    
    // Cor do céu como fundo padrão
    var sky_color = vec3<f32>(0.3, 0.5, 0.8);
    var color = sky_color;
    
    // Se o raio não bater na "caixa do mundo" ou a caixa estiver atrás de nós, desenhe o céu.
    if (t_far < max(t_near, 0.0)) {
        return vec4<f32>(sky_color, 1.0);
    }

    // Se a câmera estiver fora do mundo, "teleporta" o raio direto para a borda do mundo!
    if (t_near > 0.0) {
        ro = ro + rd * (t_near + 0.001); // Avança rápido até a colisão do limite
    }

    // --- DDA OTIMIZADO (Só roda DENTRO do mundo) ---
    var map_pos = vec3<i32>(floor(ro));
    let delta_dist = abs(1.0 / rd);
    let step = vec3<i32>(sign(rd));
    var side_dist = vec3<f32>(0.0);
    
    if (rd.x < 0.0) { side_dist.x = (ro.x - f32(map_pos.x)) * delta_dist.x; } else { side_dist.x = (f32(map_pos.x) + 1.0 - ro.x) * delta_dist.x; }
    if (rd.y < 0.0) { side_dist.y = (ro.y - f32(map_pos.y)) * delta_dist.y; } else { side_dist.y = (f32(map_pos.y) + 1.0 - ro.y) * delta_dist.y; }
    if (rd.z < 0.0) { side_dist.z = (ro.z - f32(map_pos.z)) * delta_dist.z; } else { side_dist.z = (f32(map_pos.z) + 1.0 - ro.z) * delta_dist.z; }

    var hit = false;
    var side = 0; 
    let MAX_STEPS = 600; // Suficiente para cruzar o mapa visível inteiro

    for (var i = 0; i < MAX_STEPS; i++) {
        // Se saiu do limite do mundo durante o DDA, desiste
        if (map_pos.x < 0 || map_pos.x >= 512 || map_pos.y < 0 || map_pos.y >= 128 || map_pos.z < 0 || map_pos.z >= 512) {
            break;
        }

        if (get_voxel(map_pos) != 0u) {
            hit = true; 
            break;
        }

        if (side_dist.x < side_dist.y && side_dist.x < side_dist.z) {
            side_dist.x += delta_dist.x; map_pos.x += step.x; side = 0;
        } else if (side_dist.y < side_dist.z) {
            side_dist.y += delta_dist.y; map_pos.y += step.y; side = 1;
        } else {
            side_dist.z += delta_dist.z; map_pos.z += step.z; side = 2;
        }
    }

    if (hit) {
        var base_color = vec3<f32>(0.7, 0.6, 0.4); 
        let noise = f32((map_pos.x * 7 + map_pos.y * 3 + map_pos.z * 5) % 3) * 0.08;
        base_color = base_color + noise;

        if (side == 0) { color = base_color * 0.7; } 
        else if (side == 1) { color = base_color * 1.0; } 
        else { color = base_color * 0.5; }
        
        var voxel_dist = 0.0;
        if (side == 0) { voxel_dist = side_dist.x - delta_dist.x; }
        else if (side == 1) { voxel_dist = side_dist.y - delta_dist.y; }
        else { voxel_dist = side_dist.z - delta_dist.z; }
        
        // FOG REATIVADO: Mistura suavemente o terreno com o céu nas bordas!
        // Ajustamos para a névoa ficar pesada antes de acabar o limite de visão
        let fog_factor = clamp(voxel_dist / 400.0, 0.0, 1.0);
        color = mix(color, sky_color, fog_factor);
    }

    return vec4<f32>(color, 1.0);
}