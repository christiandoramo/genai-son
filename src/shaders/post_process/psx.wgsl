fn apply_psx_effects(color: vec3<f32>, frag_coord: vec2<f32>) -> vec3<f32> {
    // 1. Bayer Dithering 4x4
    let dither_mat = array<f32, 16>(
        0.0, 0.5, 0.125, 0.625,
        0.75, 0.25, 0.875, 0.375,
        0.1875, 0.6875, 0.0625, 0.5625,
        0.9375, 0.4375, 0.8125, 0.3125
    );
    let dx = u32(frag_coord.x) % 4u;
    let dy = u32(frag_coord.y) % 4u;
    let dither = dither_mat[dy * 4u + dx] - 0.5;

    // 2. Posterization (Cores 5-bit) + Injeção de Dither
    var res = color + (dither * 0.08);
    res = floor(res * 32.0) / 32.0;

    return res;
}