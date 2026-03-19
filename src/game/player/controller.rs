use crate::engine::input::keyboard::InputState;
use crate::engine::voxel::material::*;
use crate::game::player::{GameMode, Player, WeaponType};
use winit::keyboard::KeyCode;

pub fn handle_input(player: &mut Player, input: &InputState) {
    if input.just_pressed(KeyCode::KeyG) {
        player.mode = if player.mode == GameMode::God {
            GameMode::Normal
        } else {
            GameMode::God
        };
    }
    if input.just_pressed(KeyCode::KeyF) {
        player.flashlight = !player.flashlight;
    }
    if input.just_pressed(KeyCode::KeyN) {
        player.is_day = !player.is_day;
    }

    if input.just_pressed(KeyCode::Digit0) {
        player.active_weapon = WeaponType::None;
    }
    if input.just_pressed(KeyCode::Digit1) {
        player.active_weapon = WeaponType::Creator;
        player.selected_material = VOXEL_SAND;
    }
    if input.just_pressed(KeyCode::Digit2) {
        player.active_weapon = WeaponType::Creator;
        player.selected_material = VOXEL_WATER;
    }
    if input.just_pressed(KeyCode::Digit3) {
        player.active_weapon = WeaponType::Creator;
        player.selected_material = VOXEL_GAS;
    }
    if input.just_pressed(KeyCode::Digit4) {
        player.active_weapon = WeaponType::Creator;
        player.selected_material = VOXEL_DIRT;
    }
    if input.just_pressed(KeyCode::Digit5) {
        player.active_weapon = WeaponType::Plasma;
    }
    if input.just_pressed(KeyCode::Digit6) {
        player.active_weapon = WeaponType::Bazooka;
    }
}
