use bevy::prelude::*;
use crate::world::TipoBloco;

pub fn blocos_no_raio(mapa: &bevy::utils::HashMap<IVec3, TipoBloco>, pos: Vec3, raio: f32) -> bool {
    let r = (raio + 0.5).ceil() as i32;
    let cx = pos.x.round() as i32; 
    let cy = pos.y.round() as i32; 
    let cz = pos.z.round() as i32;

    let raio_sq = raio * raio;

    for x in -r..=r {
        for y in -r..=r {
            for z in -r..=r {
                let b_pos = IVec3::new(cx + x, cy + y, cz + z);
                if mapa.contains_key(&b_pos) {
                    let bx = b_pos.x as f32;
                    let by = b_pos.y as f32;
                    let bz = b_pos.z as f32;

                    let closest_x = pos.x.clamp(bx - 0.5, bx + 0.5);
                    let closest_y = pos.y.clamp(by - 0.5, by + 0.5);
                    let closest_z = pos.z.clamp(bz - 0.5, bz + 0.5);

                    let dist_sq = (pos.x - closest_x).powi(2) +
                                  (pos.y - closest_y).powi(2) +
                                  (pos.z - closest_z).powi(2);

                    if dist_sq <= raio_sq {
                        return true;
                    }
                }
            }
        }
    }
    false
}

pub fn esta_dentro_do_chao(mapa: &bevy::utils::HashMap<IVec3, TipoBloco>, pos: Vec3, up_vector: Vec3) -> bool {
    let raio = 0.28; 
    // Cápsula densa (4 esferas) para deslizar perfeitamente e não enganchar o joelho
    blocos_no_raio(mapa, pos - (up_vector * 0.85), raio) || 
    blocos_no_raio(mapa, pos - (up_vector * 0.4), raio) || 
    blocos_no_raio(mapa, pos, raio) ||
    blocos_no_raio(mapa, pos + (up_vector * 0.6), raio)
}