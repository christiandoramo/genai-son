// src/shader.wgsl

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

struct Uniforms {
    resolution: vec2<f32>,
    time: f32,
    _padding: f32,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    let x = f32((in_vertex_index << 1u) & 2u);
    let y = f32(in_vertex_index & 2u);
    out.clip_position = vec4<f32>(x * 2.0 - 1.0, 1.0 - y * 2.0, 0.0, 1.0);
    out.uv = vec2<f32>(x, y);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // 1. Centraliza a tela e ajusta a proporção (Aspect Ratio)
    var uv = in.clip_position.xy / uniforms.resolution.xy;
    uv = uv * 2.0 - 1.0; // Agora o centro da tela é (0,0)
    uv.x *= uniforms.resolution.x / uniforms.resolution.y; 
    uv.y = -uv.y; // Inverte o eixo Y para o padrão 3D clássico

    // 2. Setup da Câmera 3D
    let ro = vec3<f32>(0.0, 0.0, -3.0); // Câmera recuada no eixo Z
    let rd = normalize(vec3<f32>(uv, 1.0)); // O raio aponta para "frente" (Z positivo)

    // 3. Raymarching Loop (O Disparo do Raio)
    var t_dist = 0.0; // Distância total viajada pelo raio
    var color = vec3<f32>(0.0); // Cor de fundo (Preto)

    for (var i = 0; i < 80; i++) {
        let p = ro + rd * t_dist; // Posição atual do raio no espaço
        
        // Matemátia da Esfera: Distância até o centro (0,0,0) menos o raio (1.0)
        let d = length(p) - 1.0; 
        
        if d < 0.001 { // O raio "bateu" na superfície da esfera!
            let normal = normalize(p); // A normal aponta para fora do centro
            
            // Luz dinâmica rodando ao redor da esfera
            let light_pos = vec3<f32>(sin(uniforms.time * 2.0) * 2.0, 2.0, cos(uniforms.time * 2.0) * 2.0);
            let light_dir = normalize(light_pos - p);
            
            // Calcula a luz difusa (sombra básica)
            let diff = max(dot(normal, light_dir), 0.0);
            
            // Pinta a esfera de azul com a iluminação
            color = vec3<f32>(0.1, 0.5, 1.0) * diff + vec3<f32>(0.05); 
            break;
        }
        
        if t_dist > 100.0 { // O raio foi longe demais, paramos de calcular
            break; 
        }
        
        t_dist += d; // Avança o raio
    }

    return vec4<f32>(color, 1.0);
}