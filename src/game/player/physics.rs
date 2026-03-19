use crate::engine::input::keyboard::InputState;
use crate::game::player::math_util::*;
use crate::game::player::{GameMode, Player};
use winit::keyboard::KeyCode;

pub fn update_player(player: &mut Player, input: &InputState, dt: f32) {
    if player.mode == GameMode::God {
        update_god_mode(player, input, dt);
    } else {
        update_survival(player, input, dt);
    }
}

fn update_god_mode(player: &mut Player, input: &InputState, dt: f32) {
    let speed = 80.0 * dt;
    let front = player.camera.get_front();
    let right = player.camera.get_right();

    let up = normalize_or_zero(cross(right, front)); // Devolve 100% de voo livre como um Drone 3D!

    let mut dir = [0.0, 0.0, 0.0];

    if input.is_pressed(KeyCode::KeyW) {
        dir[0] += front[0];
        dir[1] += front[1];
        dir[2] += front[2];
    }
    if input.is_pressed(KeyCode::KeyS) {
        dir[0] -= front[0];
        dir[1] -= front[1];
        dir[2] -= front[2];
    }
    if input.is_pressed(KeyCode::KeyA) {
        dir[0] -= right[0];
        dir[1] -= right[1];
        dir[2] -= right[2];
    }
    if input.is_pressed(KeyCode::KeyD) {
        dir[0] += right[0];
        dir[1] += right[1];
        dir[2] += right[2];
    }
    if input.is_pressed(KeyCode::KeyE) {
        dir[0] += up[0];
        dir[1] += up[1];
        dir[2] += up[2];
    }
    if input.is_pressed(KeyCode::KeyQ) {
        dir[0] -= up[0];
        dir[1] -= up[1];
        dir[2] -= up[2];
    }

    let dir_norm = normalize_or_zero(dir);
    player.camera.pos[0] += dir_norm[0] * speed;
    player.camera.pos[1] += dir_norm[1] * speed;
    player.camera.pos[2] += dir_norm[2] * speed;
}

fn update_survival(player: &mut Player, input: &InputState, dt: f32) {
    let center = [128.0, 128.0, 128.0];
    let rel_pos = [
        player.camera.pos[0] - center[0],
        player.camera.pos[1] - center[1],
        player.camera.pos[2] - center[2],
    ];
    let under_gravity = length(rel_pos) < 200.0;

    if under_gravity {
        let bias = 2.0;
        let abs_x = rel_pos[0].abs()
            + if player.physics_up[0].abs() > 0.5 {
                bias
            } else {
                0.0
            };
        let abs_y = rel_pos[1].abs()
            + if player.physics_up[1].abs() > 0.5 {
                bias
            } else {
                0.0
            };
        let abs_z = rel_pos[2].abs()
            + if player.physics_up[2].abs() > 0.5 {
                bias
            } else {
                0.0
            };

        player.physics_up = if abs_x >= abs_y && abs_x >= abs_z {
            [rel_pos[0].signum(), 0.0, 0.0]
        } else if abs_y >= abs_x && abs_y >= abs_z {
            [0.0, rel_pos[1].signum(), 0.0]
        } else {
            [0.0, 0.0, rel_pos[2].signum()]
        };
    }

    let target_visual_up = if under_gravity {
        let p_norm = normalize_or_zero(rel_pos);
        normalize_or_zero([
            p_norm[0].abs().powf(2.5) * p_norm[0].signum(),
            p_norm[1].abs().powf(2.5) * p_norm[1].signum(),
            p_norm[2].abs().powf(2.5) * p_norm[2].signum(),
        ])
    } else {
        [0.0, 1.0, 0.0]
    };

    player.visual_up = normalize_or_zero(slerp(player.visual_up, target_visual_up, dt * 8.0));
    player.camera.reorient(player.visual_up);

    let speed = 15.0 * dt;
    let local_fwd = player.camera.local_forward;
    let right = player.camera.get_right();
    let mut dir = [0.0, 0.0, 0.0];

    if input.is_pressed(KeyCode::KeyW) {
        dir[0] += local_fwd[0];
        dir[1] += local_fwd[1];
        dir[2] += local_fwd[2];
    }
    if input.is_pressed(KeyCode::KeyS) {
        dir[0] -= local_fwd[0];
        dir[1] -= local_fwd[1];
        dir[2] -= local_fwd[2];
    }
    if input.is_pressed(KeyCode::KeyA) {
        dir[0] -= right[0];
        dir[1] -= right[1];
        dir[2] -= right[2];
    }
    if input.is_pressed(KeyCode::KeyD) {
        dir[0] += right[0];
        dir[1] += right[1];
        dir[2] += right[2];
    }

    let dir_norm = normalize_or_zero(dir);
    let mut move_delta = [
        dir_norm[0] * speed,
        dir_norm[1] * speed,
        dir_norm[2] * speed,
    ];
    let dot_up = dot(move_delta, player.physics_up);
    move_delta[0] -= dot_up * player.physics_up[0];
    move_delta[1] -= dot_up * player.physics_up[1];
    move_delta[2] -= dot_up * player.physics_up[2];

    let mut next_pos = player.camera.pos;
    let step_up = [
        player.physics_up[0] * 0.1,
        player.physics_up[1] * 0.1,
        player.physics_up[2] * 0.1,
    ];

    next_pos[0] += move_delta[0];
    if is_colliding(
        player,
        [
            next_pos[0] + step_up[0],
            next_pos[1] + step_up[1],
            next_pos[2] + step_up[2],
        ],
    ) {
        next_pos[0] -= move_delta[0];
    }
    next_pos[1] += move_delta[1];
    if is_colliding(
        player,
        [
            next_pos[0] + step_up[0],
            next_pos[1] + step_up[1],
            next_pos[2] + step_up[2],
        ],
    ) {
        next_pos[1] -= move_delta[1];
    }
    next_pos[2] += move_delta[2];
    if is_colliding(
        player,
        [
            next_pos[0] + step_up[0],
            next_pos[1] + step_up[1],
            next_pos[2] + step_up[2],
        ],
    ) {
        next_pos[2] -= move_delta[2];
    }
    player.camera.pos = next_pos;

    if under_gravity {
        player.velocity_y -= 25.0 * dt;
    } else {
        player.velocity_y *= 0.9;
    }
    let grav_delta = [
        player.physics_up[0] * player.velocity_y * dt,
        player.physics_up[1] * player.velocity_y * dt,
        player.physics_up[2] * player.velocity_y * dt,
    ];

    next_pos = player.camera.pos;
    next_pos[0] += grav_delta[0];
    next_pos[1] += grav_delta[1];
    next_pos[2] += grav_delta[2];
    if is_colliding(player, next_pos) {
        player.velocity_y = 0.0;
        player.on_ground = true;
    } else {
        player.camera.pos = next_pos;
        player.on_ground = false;
    }

    // if input.is_pressed(KeyCode::Space) && player.on_ground {
    //     player.velocity_y = 10.0;
    //     player.on_ground = false;
    // }

    if input.just_pressed(KeyCode::Space) && player.on_ground {
        player.velocity_y = 10.0;
        player.on_ground = false;
    }
}

pub fn is_colliding(player: &Player, pos: [f32; 3]) -> bool {
    let radius = 0.25;
    let up = player.visual_up;
    let points = [
        [
            pos[0] + up[0] * 0.2,
            pos[1] + up[1] * 0.2,
            pos[2] + up[2] * 0.2,
        ],
        pos,
        [
            pos[0] - up[0] * 1.4,
            pos[1] - up[1] * 1.4,
            pos[2] - up[2] * 1.4,
        ],
    ];

    for p in points {
        let min_x = (p[0] - radius).floor() as i32;
        let max_x = (p[0] + radius).ceil() as i32;
        let min_y = (p[1] - radius).floor() as i32;
        let max_y = (p[1] + radius).ceil() as i32;
        let min_z = (p[2] - radius).floor() as i32;
        let max_z = (p[2] + radius).ceil() as i32;
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                for z in min_z..=max_z {
                    if let Some(&v) = player.world_edits.get(&[x, y, z]) {
                        if v != 0 && v != 2 {
                            return true;
                        }
                        continue;
                    }
                    if gpu_noise_mirror::is_voxel_solid(x, y, z) {
                        return true;
                    }
                }
            }
        }
    }
    false
}

mod gpu_noise_mirror {
    pub fn is_voxel_solid(x: i32, y: i32, z: i32) -> bool {
        if x < 0 || x > 255 || y < 0 || y > 255 || z < 0 || z > 255 {
            return false;
        }
        let px = x as f32 - 128.0;
        let py = y as f32 - 128.0;
        let pz = z as f32 - 128.0;
        let d = (px * px * px * px + py * py * py * py + pz * pz * pz * pz)
            .sqrt()
            .sqrt();
        if d > 80.0 {
            return false;
        }
        let dir = crate::game::player::math_util::normalize_or_zero([px, py, pz]);

        let cont = noise_3d(dir[0] * 1.2, dir[1] * 1.2, dir[2] * 1.2);
        let colinas = noise_3d(dir[0] * 3.0, dir[1] * 3.0, dir[2] * 3.0).max(0.0);
        let detalhes = noise_3d(dir[0] * 6.0, dir[1] * 6.0, dir[2] * 6.0).max(0.0);
        let mut h = 40.0 + (cont * 10.0);
        if cont > -0.1 {
            h += colinas * 12.0 + detalhes * 4.0; // O detalhes * 4.0 estava na CPU mas foi apagado na GPU!
        }

        if d > (h / 2.0).round() * 2.0 {
            return false;
        }
        let cave = noise_3d(px * 0.08, py * 0.08, pz * 0.08).abs();
        if cave < 0.05 && d > 25.0 {
            return false;
        }
        true
    }
    fn noise_3d(x: f32, y: f32, z: f32) -> f32 {
        let (px, py, pz) = (x.floor(), y.floor(), z.floor());
        let (fx, fy, fz) = (x - px, y - py, z - pz);
        let (ux, uy, uz) = (
            fx * fx * (3.0 - 2.0 * fx),
            fy * fy * (3.0 - 2.0 * fy),
            fz * fz * (3.0 - 2.0 * fz),
        );
        let mix = |a: f32, b: f32, t: f32| -> f32 { a + (b - a) * t };

        // AQUI A CORREÇÃO: Tipamos ax, ay, az como f32 e usamos 0.1031_f32
        let h = |ax: f32, ay: f32, az: f32| -> f32 {
            let mut p3x = (ax * 0.1031_f32).fract();
            let mut p3y = (ay * 0.1031_f32).fract();
            let mut p3z = (az * 0.1031_f32).fract();
            let d = p3x * (p3y + 33.33_f32) + p3y * (p3z + 33.33_f32) + p3z * (p3x + 33.33_f32);
            p3x += d;
            p3y += d;
            p3z += d;
            ((p3x + p3y) * p3z).fract() * 2.0_f32 - 1.0_f32
        };

        mix(
            mix(
                mix(h(px, py, pz), h(px + 1.0, py, pz), ux),
                mix(h(px, py + 1.0, pz), h(px + 1.0, py + 1.0, pz), ux),
                uy,
            ),
            mix(
                mix(h(px, py, pz + 1.0), h(px + 1.0, py, pz + 1.0), ux),
                mix(
                    h(px, py + 1.0, pz + 1.0),
                    h(px + 1.0, py + 1.0, pz + 1.0),
                    ux,
                ),
                uy,
            ),
            uz,
        )
    }
}
