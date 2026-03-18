pub fn dot(a: [f32; 3], b: [f32; 3]) -> f32 { a[0]*b[0] + a[1]*b[1] + a[2]*b[2] }
pub fn cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] { [ a[1]*b[2] - a[2]*b[1], a[2]*b[0] - a[0]*b[2], a[0]*b[1] - a[1]*b[0] ] }
pub fn length(v: [f32; 3]) -> f32 { (v[0]*v[0] + v[1]*v[1] + v[2]*v[2]).sqrt() }

pub fn normalize_or_zero(v: [f32; 3]) -> [f32; 3] {
    let len = length(v);
    if len < 0.00001 { [0.0, 0.0, 0.0] } else { [v[0]/len, v[1]/len, v[2]/len] }
}

pub fn slerp(a: [f32; 3], b: [f32; 3], t: f32) -> [f32; 3] {
    let d = dot(a, b).clamp(-1.0, 1.0);
    if d > 0.9995 { return normalize_or_zero([a[0] + (b[0] - a[0])*t, a[1] + (b[1] - a[1])*t, a[2] + (b[2] - a[2])*t]); }
    let theta = d.acos() * t;
    let rel = normalize_or_zero([b[0] - a[0]*d, b[1] - a[1]*d, b[2] - a[2]*d]);
    [
        a[0]*theta.cos() + rel[0]*theta.sin(),
        a[1]*theta.cos() + rel[1]*theta.sin(),
        a[2]*theta.cos() + rel[2]*theta.sin()
    ]
}

pub fn rotate_vector(v: [f32; 3], k: [f32; 3], angle: f32) -> [f32; 3] {
    let cos_t = angle.cos(); let sin_t = angle.sin(); let c_kv = cross(k, v); let d_kv = dot(k, v);
    [ 
        v[0] * cos_t + c_kv[0] * sin_t + k[0] * d_kv * (1.0 - cos_t), 
        v[1] * cos_t + c_kv[1] * sin_t + k[1] * d_kv * (1.0 - cos_t), 
        v[2] * cos_t + c_kv[2] * sin_t + k[2] * d_kv * (1.0 - cos_t) 
    ]
}