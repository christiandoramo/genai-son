use winit::keyboard::KeyCode;
use crate::engine::input::keyboard::InputState;
use crate::game::player::{Player, WeaponType, GameMode};
use crate::engine::voxel::material::*;

pub fn handle_input(player: &mut Player, input: &InputState) {
    // Alternar Modos
    if input.is_pressed(KeyCode::KeyG) { player.mode = GameMode::God; }
    if input.is_pressed(KeyCode::KeyF) { player.flashlight = !player.flashlight; }
    if input.is_pressed(KeyCode::KeyN) { player.is_day = !player.is_day; }

    // Seleção de Arsenal
    if input.is_pressed(KeyCode::Digit0) { player.active_weapon = WeaponType::None; }
    if input.is_pressed(KeyCode::Digit1) { 
        player.active_weapon = WeaponType::Creator; 
        player.selected_material = VOXEL_SAND; 
    }
    if input.is_pressed(KeyCode::Digit2) { 
        player.active_weapon = WeaponType::Creator; 
        player.selected_material = VOXEL_WATER; 
    }
    if input.is_pressed(KeyCode::Digit3) { 
        player.active_weapon = WeaponType::Creator; 
        player.selected_material = VOXEL_GAS; 
    }
    if input.is_pressed(KeyCode::Digit4) { 
        player.active_weapon = WeaponType::Creator; 
        player.selected_material = VOXEL_DIRT; 
    }
    if input.is_pressed(KeyCode::Digit5) { player.active_weapon = WeaponType::Plasma; }
    if input.is_pressed(KeyCode::Digit6) { player.active_weapon = WeaponType::Bazooka; }
}