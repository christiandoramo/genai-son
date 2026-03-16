// src/shader.wgsl

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    // Truque do Triângulo de Tela Cheia: Gera vértices sem Vertex Buffer
    let x = f32((in_vertex_index << 1u) & 2u);
    let y = f32(in_vertex_index & 2u);
    
    out.clip_position = vec4<f32>(x * 2.0 - 1.0, 1.0 - y * 2.0, 0.0, 1.0);
    out.uv = vec2<f32>(x, y);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Aqui nascerá o Raycaster. 
    // Por enquanto, provamos que a tela é nossa com matemática pura.
    let r = in.uv.x;
    let g = in.uv.y;
    let b = 0.5 + 0.5 * sin(in.uv.x * 20.0 + in.uv.y * 20.0);
    
    return vec4<f32>(r, g, b, 1.0);
}