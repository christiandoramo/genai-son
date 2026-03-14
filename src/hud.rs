use bevy::prelude::*;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use crate::player::Player;

#[derive(Component)]
pub struct HudText;

pub fn setup_hud(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "Carregando Sistemas...",
            TextStyle {
                font_size: 20.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(15.0),
            left: Val::Px(15.0),
            ..default()
        }),
        HudText,
    ));
}

pub fn atualizar_hud(
    diagnostics: Res<DiagnosticsStore>,
    player_query: Query<&Player>,
    mut text_query: Query<&mut Text, With<HudText>>,
) {
    let mut fps = 0.0;
    if let Some(fps_diagnostic) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(fps_smoothed) = fps_diagnostic.smoothed() {
            fps = fps_smoothed;
        }
    }

    if let Ok(player) = player_query.get_single() {
        if let Ok(mut text) = text_query.get_single_mut() {
            let modo = if player.god_mode { "DEUS" } else { "SOBREVIVÊNCIA" };
            text.sections[0].value = format!(
                "FPS: {:.1}\nModo: {}\nVelocidade Voo (Scroll): {:.1}\nControles: W,A,S,D | F (GodMode) | Q/E (Subir/Descer)",
                fps, modo, player.god_speed
            );
        }
    }
}