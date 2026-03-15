// src/hud/mod.rs
use crate::player::Player;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        let sys = System::new_with_specifics(
            RefreshKind::new()
                .with_cpu(CpuRefreshKind::everything())
                .with_memory(MemoryRefreshKind::everything()),
        );
        app.insert_resource(SystemMonitor(sys))
            .add_systems(Startup, setup_hud)
            .add_systems(Update, atualizar_hud);
    }
}

#[derive(Resource)]
pub struct SystemMonitor(pub System);

#[derive(Component)]
pub struct HudText;

fn setup_hud(mut commands: Commands) {
    let style_rotulo = TextStyle {
        font_size: 16.0,
        color: Color::WHITE,
        ..default()
    };
    let style_valor = TextStyle {
        font_size: 16.0,
        color: Color::srgb(0.0, 1.0, 0.5),
        ..default()
    };

    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Px(10.0),
                padding: UiRect::all(Val::Px(15.0)),
                ..default()
            },
            background_color: Color::srgba(0.0, 0.0, 0.0, 0.75).into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_sections([
                    TextSection::new(
                        "[ GENAI SON ENGINE ]\n\n",
                        TextStyle {
                            font_size: 18.0,
                            color: Color::srgb(0.7, 0.5, 1.0),
                            ..default()
                        },
                    ),
                    TextSection::new("[SISTEMA]\nFPS: ", style_rotulo.clone()),
                    TextSection::new("0.0\n", style_valor.clone()),
                    TextSection::new("CPU: ", style_rotulo.clone()),
                    TextSection::new("0.0%\n", style_valor.clone()),
                    TextSection::new("RAM: ", style_rotulo.clone()),
                    TextSection::new("0.0 GB\n", style_valor.clone()),
                    TextSection::new("\n[JOGADOR]\nModo: ", style_rotulo.clone()),
                    TextSection::new("INICIANDO\n", style_valor.clone()),
                    TextSection::new("Controles: ", style_rotulo.clone()),
                    TextSection::new("WASD | F (GodMode)\n", style_valor.clone()),
                    TextSection::new("Velocidade Voo: ", style_rotulo.clone()),
                    TextSection::new("0.0\n", style_valor.clone()),
                ]),
                HudText,
            ));
        });
}

fn atualizar_hud(
    diagnostics: Res<DiagnosticsStore>,
    player_query: Query<&Player>,
    mut text_query: Query<&mut Text, With<HudText>>,
    mut monitor: ResMut<SystemMonitor>,
) {
    monitor
        .0
        .refresh_cpu_specifics(CpuRefreshKind::everything());
    monitor
        .0
        .refresh_memory_specifics(MemoryRefreshKind::everything());

    let mut fps = 0.0;
    if let Some(fps_diagnostic) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(fps_smoothed) = fps_diagnostic.smoothed() {
            fps = fps_smoothed;
        }
    }

    let cpu_usage = monitor.0.global_cpu_info().cpu_usage();
    let ram_used = monitor.0.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0;

    if let Ok(player) = player_query.get_single() {
        if let Ok(mut text) = text_query.get_single_mut() {
            text.sections[2].value = format!("{:.1}\n", fps);
            text.sections[4].value = format!("{:.1}%\n", cpu_usage);
            text.sections[6].value = format!("{:.1} GB\n", ram_used);
            text.sections[8].value = if player.god_mode {
                "DEUS (Voo Livre)\n".into()
            } else {
                "SOBREVIVENCIA\n".into()
            };
            text.sections[12].value = format!("{:.1}\n", player.god_speed);

            let cor_alerta = if fps < 30.0 || cpu_usage > 90.0 {
                Color::srgb(1.0, 0.3, 0.3)
            } else {
                Color::srgb(0.0, 1.0, 0.5)
            };
            text.sections[2].style.color = cor_alerta;
            text.sections[4].style.color = cor_alerta;
        }
    }
}
