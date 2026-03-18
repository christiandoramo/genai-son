pub const PLANET_RADIUS: f32 = 40.0;
pub const PLANET_CENTER: [f32; 3] = [128.0, 128.0, 128.0];

pub fn is_within_atmosphere(pos: [f32; 3]) -> bool {
    let dx = pos[0] - PLANET_CENTER[0];
    let dy = pos[1] - PLANET_CENTER[1];
    let dz = pos[2] - PLANET_CENTER[2];
    (dx*dx + dy*dy + dz*dz).sqrt() < (PLANET_RADIUS + 40.0)
}