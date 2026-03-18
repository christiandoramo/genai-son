struct Uniforms { resolution: vec2<f32>, time: f32, action: u32, camera_pos: vec3<f32>, flashlight_on: u32, camera_front: vec3<f32>, _padding3: f32, };
@group(0) @binding(0) var<uniform> uniforms: Uniforms;

struct WorldBuffer { data: array<u32>, };
@group(0) @binding(1) var<storage, read_write> world: WorldBuffer;
@group(0) @binding(2) var<storage, read_write> macro_world: WorldBuffer;

// Funções de Ruído Pseudoaleatório da GPU (Simulando o OpenSimplex)
fn hash(p: vec3<f32>) -> f32 {
    var p3 = fract(p * 0.1031);
    p3 += dot(p3, p3.yzx + 33.33);
    return fract((p3.x + p3.y) * p3.z) * 2.0 - 1.0; // Retorna entre -1.0 e 1.0
}
fn noise(x: vec3<f32>) -> f32 {
    let p = floor(x); let f = fract(x);
    let u = f * f * (3.0 - 2.0 * f);
    return mix(mix(mix(hash(p + vec3<f32>(0.0,0.0,0.0)), hash(p + vec3<f32>(1.0,0.0,0.0)), u.x),
                   mix(hash(p + vec3<f32>(0.0,1.0,0.0)), hash(p + vec3<f32>(1.0,1.0,0.0)), u.x), u.y),
               mix(mix(hash(p + vec3<f32>(0.0,0.0,1.0)), hash(p + vec3<f32>(1.0,0.0,1.0)), u.x),
                   mix(hash(p + vec3<f32>(0.0,1.0,1.0)), hash(p + vec3<f32>(1.0,1.0,1.0)), u.x), u.y), u.z);
}
fn get_index(x: u32, y: u32, z: u32) -> u32 { return x + y * 256u + z * 256u * 256u; }
fn get_macro_index(x: u32, y: u32, z: u32) -> u32 { let mx = x >> 3u; let my = y >> 3u; let mz = z >> 3u; return mx + my * 32u + mz * 32u * 32u; }

@compute @workgroup_size(4, 4, 4) // Agora usamos fatias muito menores e mais seguras!
fn main_gen(@builtin(global_invocation_id) id: vec3<u32>) {
// Na primeira linha do main_gen:
    if (id.x >= 256u || id.y >= 256u || id.z >= 256u) { return; }

    // O novo centro:
    let cx = 128.0; let cy = 128.0; let cz = 128.0;
    
    let px = f32(id.x) - cx;
    let py = f32(id.y) - cy;
    let pz = f32(id.z) - cz;

    // MATEMÁTICA DO SUPERELIPSOIDE (A Cúbica Esférica do seu código antigo!)
    // dist_base = (pos.x.powi(4) + pos.y.powi(4) + pos.z.powi(4)).powf(0.25);
    let dist_base = sqrt(sqrt((px*px*px*px) + (py*py*py*py) + (pz*pz*pz*pz)));

    let planet_radius = 40.0;

    if (dist_base > planet_radius + 40.0) { return; } // Limite do céu (Vazio)

    let dir = normalize(vec3<f32>(px, py, pz));
    let nx = dir.x; let ny = dir.y; let nz = dir.z;

    // =========================================================================
    // TRADUÇÃO EXATA DO BIOMA REFINADO
    // =========================================================================
    let continentes = noise(vec3<f32>(nx * 1.2, ny * 1.2, nz * 1.2));
    let colinas = max(noise(vec3<f32>(nx * 3.0, ny * 3.0, nz * 3.0)), 0.0);
    let detalhes = max(noise(vec3<f32>(nx * 6.0, ny * 6.0, nz * 6.0)), 0.0);

    var altura_superficie = planet_radius + (continentes * 10.0);

    if (continentes > -0.1) {
        altura_superficie += colinas * 12.0;
        altura_superficie += detalhes * 4.0;
    }

    let superficie = round(altura_superficie / 2.0) * 2.0;
    let nivel_mar = planet_radius + 0.0;

    var material = 0u;

    if (dist_base <= superficie) {
        let profundidade = superficie - dist_base;

        if (continentes < -0.2 && dist_base <= nivel_mar + 1.5) {
            material = 1u; // Areia
        } else if (continentes > 0.4 && superficie > planet_radius + 15.0) {
            if (profundidade < 1.0) { material = 4u; } // Neve (Usando ID 4 para representar branco/neve)
            else { material = 8u; } // Pedra
        } else {
            if (profundidade < 1.0) { material = 7u; } // Grama
            else if (profundidade < 3.0) { material = 1u; } // Areia
            else { material = 8u; } // Pedra
        }
    } else if (dist_base <= nivel_mar) {
        material = 2u; // Água
    }

    // CAVERNAS VOLUMÉTRICAS
    let cave_noise = abs(noise(vec3<f32>(px * 0.08, py * 0.08, pz * 0.08)));
    if (cave_noise < 0.05 && dist_base > planet_radius - 15.0) {
        material = 0u; // Esculpe a caverna
    }

    if (dist_base < planet_radius - 15.0) { material = 9u; } // Capa de Magma
    if (dist_base < planet_radius - 20.0) { material = 10u; } // Núcleo Sólido de Ferro

    let idx = get_index(id.x, id.y, id.z);
    world.data[idx] = material;
    if (material != 0u) { macro_world.data[get_macro_index(id.x, id.y, id.z)] = 1u; }
}