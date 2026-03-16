// src/main.rs
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PresentMode, WindowResolution};

mod camera;
mod hud;
mod physics;
mod player;
mod world;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "PROTÓTIPO - GENAI SON".into(),
                resolution: WindowResolution::new(1024.0, 768.0),
                present_mode: PresentMode::AutoNoVsync, 
                ..default()
            }),
            ..default()
        }))
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins((
            camera::CameraPlugin,
            hud::HudPlugin,
            world::WorldPlugin,
            player::PlayerPlugin,
        ))
        // Diminuímos a luz ambiente para as sombras ficarem mais visíveis
        .insert_resource(ClearColor(Color::srgb(0.4, 0.7, 0.9))) // Azul claro
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 80.0, 
        })
        .add_systems(Startup, setup_luzes) // <--- Chamando o nosso Sol
        .add_systems(Update, gerenciar_cursor)
        .run();
}

// O nosso Sol direcional que projeta sombras perfeitas nos Voxels
fn setup_luzes(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 1500.0, // Brilho do Sol
            shadows_enabled: true, // A mágica acontece aqui
            ..default()
        },
        // Inclinamos o sol para as sombras ficarem diagonais e bonitas
        transform: Transform::from_xyz(100.0, 100.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn gerenciar_cursor(
    mut windows: Query<&mut Window>,
    input_teclado: Res<ButtonInput<KeyCode>>,
    input_mouse: Res<ButtonInput<MouseButton>>,
) {
    let mut window = windows.single_mut();
    if input_teclado.just_pressed(KeyCode::Escape) {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
    }
    if input_mouse.just_pressed(MouseButton::Left) {
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;
    }
}