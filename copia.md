========================================
README.md
========================================
OBS: Estou no passo 3 ainda, não foi concluído com sucesso, apresenta erros - o mundo as vezes fica fragmentado, não renderiza direito, as vezes só vê um horizonte chegio de faces laterais soltas dos cubos do planeta sem sentido, talvez devido ao gerenciamento dos chunks, veja o que pode fazer. O restante do código já feito está ok, não precisa se preocupar.

# GenAI Son: Metarealidade Procedural Voxel 🌌

**Visão Geral do Projeto:**
Este projeto é a construção de um motor de jogo Voxel 3D de alta performance do zero, usando Rust e Bevy. O objetivo final (não é fazer um clone de minecraft) é criar um "Metarealidade" ou "Motor de Areia" (inspirado em *Noita* e *No Man's Sky*), onde tudo é gerado proceduralmente por algoritmos matemáticos e IA: terrenos, gravidade, dinâmica de fluidos, trilha sonora e o comportamento de NPCs. 

O propósito desse jogo é usar ao máximo da inteligência (conhecimentos em computação, programação, jogos, biologia, animação procedural, criação de npcs procedruralmente,física, geometria euclidiana e não euclidiana, codificação de shaders e formas, matemática,engenharia, shaders, música , produção musical procedural,PCG, IA) mais ampla e mais aprofundade do mundo para fazer um jogo codificado por esse própria inteligência (milhares de anos de milhares dos melhores cientistas do mundo estão armazenadas na sua "cabeçona", não á estúdio AAA ou dev gênio que ganhe de você evido a sua diversidade e profundidade de conhecimentos "inumanos"-impossível para um humano ter tanto conhecimento), que é a sua como GenAI!

Tudo que fiz até agora no meu jogo foi criar um mundo procedural voxelizado que é um planeta Superelipsoide (cubo esférico), com movimentação fps 3d, personagem tem gravidade e pulo nesse planetinha.


Inspirações: 
- Minecraft: Mundos com biomas, relevos / estruturas geológicas gerados aleatoriamente (planetas ao longo do espaço)
- Teardown: Destructibilidade do mundo é essencial
- No Man's Sky: Viagem literal pelo espaço, e entrar em orbitais de incontáveis (quase infinitos de tantos planetas) planetas procedurais
- Noita: Motor de areia, os pixels (no nosso caso voxels) se comportam com base em um "motor de areia", além da infita destructibilidade, há reações diferentes em ambientes diferentes baseado em mecânica (ex: explosões), termodinâmica (ex: calor do fogo de explosões) e química.
- GTA: Rotinas procedurais com pessoas e animais nas cidades grandes (planetas nesse caso). Liberdade para bater e eliminar tudo e todos.

**Regra de Ouro da Arte:** NENHUM ativo pré-fabricado (modelos 3D, áudios .mp3/.wav, texturas .png) deve ser usado se puder ser gerado proceduralmente ou matematicamente na memória RAM - para isso o estilo artístico é o cúbico/voxelizado e minimalismo, facilitando esse processo de gerações diretas em memória RAM.

---

## 🛠️ Stack Tecnológica

### Meu PC: Relatório de detalhes do sistema

#### Informações de hardware:
- **Modelo do hardware:**                          Dell Inc. Latitude 3410
- **Memória:**                                     16,0 GiB
- **Processador:**                                 Intel® Core™ i7-10510U × 8
- **Gráficos:**                                    Intel® UHD Graphics (CML GT2)
- **Capacidade de disco:**                         (null)

#### Informações de software:
- **Versão do firmware:**                          1.31.0
- **Nome do SO:**                                  Ubuntu 24.04.4 LTS
- **Compilação do SO:**                            (null)
- **Tipo do SO:**                                  64 bits
- **Versão do GNOME:**                             46
- **Sistema de janelas:**                          Wayland
- **Verificação do kernel:**                       Linux 6.8.0-101-generic

### VS code extensions:
- rust-analyzer: intellisense para rust (linguagem de programação)
- CodeLLDB: debugger para rust
- WGSL: intellisense para WGSL (linguagem dos shaders)
- Even Better TOML: intellisense para TOML (linguagem do gerenciador do projeto)

### Cargo.toml atual

- **Linguagem:** Rust (Foco máximo em performance e paralelismo).
  - rustc 1.94.0 (4a4ef493e 2026-03-02)
- **Engine:** Bevy (Arquitetura ECS - Entity Component System).
- **Matemática/Terreno:** `noise` (Perlin Noise para geração pseudoaleatória).
- *Física:* Customizada via Swept AABB (sem bibliotecas externas pesadas).
- *Áudio (Futuro):* Síntese procedural na RAM (Kira) - Não será usado por enquanto.

```toml
[package]
name = "genai_son"
version = "0.1.0"
edition = "2024"

[dependencies]
# O núcleo da Engine (Performance absurda e ECS)
bevy = { version = "0.14", features = ["dynamic_linking"] }

#será usado no futuro para dar vida em músicas procedurais na metarealidade
# kira (também usar fundsp) = "0.9" # não será usado no momento,mas o propósito é o mundo gerar músicas procedurais em tempo real com base no jogo

# Matemática, Ruído e Geração de Universos
noise = "0.9" # Para gerar montanhas e biomas

[profile.dev]
opt-level = 1
[profile.dev.package."*"]
opt-level = 3
```
---

## 📂 Código mais atual do Projeto

O código-fonte está estruturado seguindo o padrão ECS (Entity Component System) da Bevy Engine, isolando responsabilidades para facilitar a geração procedural:

* **`main.rs`**: O coração do motor. Configura as janelas, desabilita limitadores de hardware (VSync) e inicializa a cascata de plugins de todos os outros módulos.
* **`camera/mod.rs`**: Controlador do sistema óptico. Gerencia o rig da câmera principal, configurações de distância de renderização, "neblina" atmosférica e a sincronização visual com a rotação da cabeça do jogador.
* **`hud/mod.rs`**: Sistema de interface de usuário (UI). Faz a ponte de telemetria em tempo real, extraindo dados de diagnóstico da engine e dos componentes para mostrar o FPS dinâmico e o estado da simulação.
* **`physics/mod.rs`**: A biblioteca matemática do "Motor Sand". Evita motores físicos externos genéricos, implementando o nosso algoritmo exclusivo *Minkowski Sweep AABB*. É ele que calcula a colisão contínua de esferas contra a malha Voxel 3D, permitindo deslizar por quinas esféricas e gravidade customizada.
* **`player/`**: Lida com a entidade que navega pelo mundo.
  * **`mod.rs`**: Contêiner do plugin principal e das constantes da força gravitacional.
  * **`camera.rs`**: Motor de captura crua do mouse para cálculo delta da visão.
  * **`movement.rs`**: Uma máquina de estados complexa que engloba a "Mecânica de Sobrevivência" (cálculo de vetores euclidianos curvados para andar no planeta superesferóide) e o "Modo Deus" (física de voo irrestrito).
* **`world/`**: O cérebro procedural (A matriz).
  * **`mod.rs`**: Registro das memórias estáticas (HashMaps e HashSets da RAM).
  * **`generator.rs`**: Algoritmos matemáticos de PCG (*Procedural Content Generation*), orquestrando funções de Perlin/Simplex Noise para talhar biomas e montanhas matematicamente e gerenciar a fila da CPU de Chunks.
  * **`mesher.rs`**: O forjador da placa de vídeo. Pega dados matemáticos invisíveis da RAM e processa algoritmos *Greedy Meshing* cruéis para fundir milhões de faces inúteis, entregando polígonos ultra-otimizados para a GPU desenhar.

---

## 🚀 Próximos Passos (Roadmap)
Para a IA que pegar o projeto agora e for continuar o desenvolvimento a partir daqui, os próximos focos são:

1. **Refatoração e melhorias:** Os códigos já estão 100% funcionais, cuidado para não modificar acidentalmente. Deve melhorar a separação de arquivos. Uma melhoria que pode fazer é colocar um raio de órbita no planeta (pode ser extremamente distante), para quando o jogador se aproximar a gravidade fazer efeito (cuidado que teram infindáveis planetas, logo deve pensar de forma que não deixe pesado, talvez transferir responsabilidade da física para o player, como já está agora?)
2. **Otimização:** Atualmente o jogo está bem lendo, 10-30 de fps e diminui conforme aumento do raio do planeta (Superelipsoide). Devem ser feitas otimizações, por exemplo algoritmo complexo chamado Greedy Meshing que funde milhares de blocos em um único "objeto" 3D. No momento nós estamos mandando a placa de vídeo desenhar e calcular a sombra de 30.000 cubos separados por frame, isso é horrível. Precisaremos acrescentar centenas de linhas de código de mesclagem para melhorar isso - as otimizações passadas foram apenas reduzir o raio do planeta, o que não faz sentido.
3. **Geração Dinâmica (Chunks Finitos-Discretos):** O mundo atual tem um `GRID_SIZE` fixo. Precisamos implementar *Chunks* que carregam e descarregam conforme o jogador anda, mas com muito cuidado, pois o mundo não é plano - isso ainda não está sendo feito. Deve ter um bom campo de visão para não causar estranhamento, para isso pode refazer a névoa para realmente deixar a renderização computacional mais econômica no meu PC. E atente-se, como é um planeta, deve renderizar corretamente conforme nos afastamos, não podem ficar pedaços faltando na vista, isso seria muito errado.
4.**Criação de um mundo voxelizada para a futura física**: permitir com que os cubos do mundo possam se dividir em voxels quando forem destruídos, mas as superfícies devem continuar 100% planas em relação a física de colisões do personagem, para ele continuar "pisando" corretamente, o mundo deve continuar em cubos como já está.
5. **Dinâmica de Fluidos Voxel (Cellular Automata) - contradição, estamos em planetas cúbicos, como vamos fazer isso, mantemos os cubos ou trocamos para voxel, meu pc vai aguentar, o bevy vai mais atrapalhar ou ajudar na performance?:**
  - 5.1 fazer a física do motor sand para os voxels agora. Para testarmos depois:
  - 5.2 Fazer duas pistolas (deve alternar ao apertar 1 e 2, apertar o mesmo botão novamente desarma as mãos):
    - 5.2.1 Uma pistola que atira uma bombinha explosiva ao colidir com um solo/parede e destrói onde colide, sendo deve ter efeitos semelhantes como o jogo noite (motor de areia), os pedaços do cubo devem desintegrar, e outros pedaços voarem/cairem como num motor sand
    - 5.2.2 uma outra pistola que consiga atirar uma bolinha de água com gravidade atire,com que blocos de água ou magma atualizem seus vizinhos no *HashMap* para escorrer com a gravidade.
6. **Áudio Procedural Dinâmico:** Reintegrar motores acústicos que reagem à física do mundo, de forma musical, com sensação viciante e agradável.

---
**Nota para IAs assistentes (Meus cientistas "Cabeçudos"):** Mantenha as otimizações $O(1)$ no HashMap. Não introduza colisões de polígonos. Confie na matemática da grade. Mantenha um comentário com o path de cada arquivo no topo. Lembre-se, você não é o assistente aqui, é o Cientista cabeçudo.


========================================
CÓDIGO DO PROJETO
========================================
.
├── Cargo.lock
├── Cargo.toml
├── copia.md
├── copia.sh
├── README.md
└── src
    ├── camera
    │   └── mod.rs
    ├── hud
    │   └── mod.rs
    ├── main.rs
    ├── physics
    │   └── mod.rs
    ├── player
    │   ├── camera.rs
    │   ├── god_mode.rs
    │   ├── mod.rs
    │   └── movement.rs
    └── world
        ├── generator.rs
        ├── mesher.rs
        └── mod.rs

7 directories, 16 files



========================================
CÓDIGO FONTE (Diretório src/)
========================================


--- Caminho do Arquivo: src/hud/mod.rs ---
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



--- Caminho do Arquivo: src/camera/mod.rs ---
// src/camera/mod.rs
use crate::player::Player;
use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, sync_camera);
    }
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct CameraPivot;

pub fn construir_rig_camera(parent: &mut ChildBuilder) {
    parent
        .spawn((
            SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.7, 0.0)),
            CameraPivot,
        ))
        .with_children(|pivot| {
            pivot.spawn((
                Camera3dBundle::default(),
                FogSettings {
                    color: Color::srgb(0.4, 0.7, 0.9),
                    falloff: FogFalloff::Linear {
                        start: 30.0,
                        end: 65.0, // Névoa densa protegendo os chunks de carregar na sua cara
                    },
                    ..default()
                },
                MainCamera,
            ));
        });
}

fn sync_camera(
    player_query: Query<&Player>,
    mut pivot_query: Query<&mut Transform, With<CameraPivot>>,
) {
    if let Ok(player) = player_query.get_single() {
        if let Ok(mut pivot_transform) = pivot_query.get_single_mut() {
            pivot_transform.rotation = Quat::from_rotation_x(player.pitch);
        }
    }
}



--- Caminho do Arquivo: src/player/movement.rs ---
// src/player/movement.rs
use super::Player;
use crate::camera::construir_rig_camera;
use crate::world::{ChunkManager, VoxelWorld};
use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};
use bevy::prelude::*;

pub fn is_god_mode(query: Query<&Player>) -> bool {
    query.get_single().map(|p| p.god_mode).unwrap_or(false)
}
pub fn is_survival_mode(query: Query<&Player>) -> bool {
    query.get_single().map(|p| !p.god_mode).unwrap_or(false)
}

pub fn spawn_player(mut commands: Commands) {
    commands
        .spawn((
            SpatialBundle::from_transform(Transform::from_xyz(
                0.0,
                super::PLANET_RADIUS + 80.0,
                0.0,
            )),
            Player {
                velocidade_y: 0.0,
                no_chao: false,
                pitch: 0.0,
                yaw: 0.0,
                god_mode: false,
                god_speed: 60.0,
            },
        ))
        .with_children(|parent| {
            construir_rig_camera(parent);
        });
}

pub fn tratar_inputs_estado(
    input: Res<ButtonInput<KeyCode>>,
    mut scroll_events: EventReader<MouseWheel>,
    mut query: Query<(&mut Transform, &mut Player)>,
) {
    let Ok((transform, mut player)) = query.get_single_mut() else {
        return;
    };

    if input.just_pressed(KeyCode::KeyF) {
        player.god_mode = !player.god_mode;
        player.velocidade_y = 0.0;

        if player.god_mode {
            // CORREÇÃO BEVY 0.14: look_to agora exige o tipo estrito Dir3 para evitar bugs matemáticos
            // if let Ok(dir_forward) = Dir3::new(transform.forward().into()) {
            //     transform.look_to(dir_forward, Dir3::Y);
            // }

            let (yaw, pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
            player.yaw = yaw;
            player.pitch = pitch;
        }
    }

    for ev in scroll_events.read() {
        if player.god_mode {
            let scroll = match ev.unit {
                MouseScrollUnit::Line => ev.y * 5.0,
                MouseScrollUnit::Pixel => ev.y * 0.1,
            };
            player.god_speed = (player.god_speed + scroll).clamp(5.0, 300.0);
        }
    }
}

pub fn rotacionar_camera(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<(&mut Transform, &mut Player)>,
) {
    let Ok((mut transform, mut player)) = query.get_single_mut() else { return; };
    let mut mouse_dx = 0.0;
    let mut mouse_dy = 0.0;

    for ev in mouse_motion_events.read() {
        mouse_dx -= ev.delta.x * 0.003;
        mouse_dy -= ev.delta.y * 0.003;
    }

    player.pitch = (player.pitch + mouse_dy).clamp(-1.5, 1.5);
    // Como o corpo já está alinhado ao planeta (tanto no God Mode quanto Sobrevivência), 
    // um simples rotate_local_y resolve tudo sem bugar os eixos!
    transform.rotate_local_y(mouse_dx); 
}
pub fn movimento_sobrevivencia(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mundo: Res<VoxelWorld>,
    chunk_manager: Res<ChunkManager>,
    mut query: Query<(&mut Transform, &mut Player)>,
) {
    let Ok((mut transform, mut player)) = query.get_single_mut() else { return; };
    let dt = time.delta_seconds().min(0.05); 
    let pos_atual = transform.translation;

    let under_gravity = pos_atual.length() < super::GRAVITY_INFLUENCE_RADIUS;
    
    // ----------------------------------------------------------------
    // 1. GRAVIDADE FÍSICA (O Cubo Duro - Mantém você colado no chão)
    // ----------------------------------------------------------------
    let bias = 1.0; 
    let mut physics_up = if under_gravity {
        let up_atual_fis = if transform.up().x.abs() > 0.5 { Vec3::X * transform.up().x.signum() }
                           else if transform.up().y.abs() > 0.5 { Vec3::Y * transform.up().y.signum() }
                           else { Vec3::Z * transform.up().z.signum() };

        let abs_x = pos_atual.x.abs() + if up_atual_fis.x.abs() > 0.5 { bias } else { 0.0 };
        let abs_y = pos_atual.y.abs() + if up_atual_fis.y.abs() > 0.5 { bias } else { 0.0 };
        let abs_z = pos_atual.z.abs() + if up_atual_fis.z.abs() > 0.5 { bias } else { 0.0 };

        if abs_x > abs_y && abs_x > abs_z { Vec3::new(pos_atual.x.signum(), 0.0, 0.0) } 
        else if abs_y > abs_x && abs_y > abs_z { Vec3::new(0.0, pos_atual.y.signum(), 0.0) } 
        else { Vec3::new(0.0, 0.0, pos_atual.z.signum()) }
    } else {
        *transform.up()
    };
    if physics_up == Vec3::ZERO { physics_up = Vec3::Y; }

    // ----------------------------------------------------------------
    // 2. GRAVIDADE VISUAL (A Esfera Suave - Antecipa a quina para a Câmera)
    // ----------------------------------------------------------------
    let mut visual_up = if under_gravity {
        // Usa a direção pura do centro do planeta para inclinar a câmera aos poucos!
        let p_norm = pos_atual.normalize_or_zero();
        let p = 2.5_f32; // Esse valor faz a câmera começar a dobrar um pouco antes da borda
        Vec3::new(
            p_norm.x.abs().powf(p) * p_norm.x.signum(),
            p_norm.y.abs().powf(p) * p_norm.y.signum(),
            p_norm.z.abs().powf(p) * p_norm.z.signum(),
        ).normalize_or_zero()
    } else {
        *transform.up()
    };
    if visual_up == Vec3::ZERO { visual_up = Vec3::Y; }

    // Rotacionamos o JOGADOR com base na Gravidade VISUAL
    if under_gravity && transform.up().dot(visual_up) > -0.999 {
        let fwd: Vec3 = transform.forward().into();
        let mut proj_fwd = (fwd - fwd.dot(visual_up) * visual_up).normalize_or_zero();
        if proj_fwd == Vec3::ZERO { proj_fwd = transform.up().into(); } 

        if let (Ok(dir_fwd), Ok(dir_up)) = (Dir3::new(proj_fwd), Dir3::new(visual_up)) {
            let target_rotation = Transform::default().looking_to(dir_fwd, dir_up).rotation;
            transform.rotation = transform.rotation.slerp(target_rotation, time.delta_seconds() * 8.0);
        }
    }

    // ----------------------------------------------------------------
    // 3. MOVIMENTO E COLISÃO (Usa estritamente a Física Cúbica)
    // ----------------------------------------------------------------
    let pos_futura = pos_atual + (-physics_up * 2.0); 
    let chunk_futuro = IVec3::new(
        (pos_futura.x / crate::world::CHUNK_SIZE as f32).floor() as i32,
        (pos_futura.y / crate::world::CHUNK_SIZE as f32).floor() as i32,
        (pos_futura.z / crate::world::CHUNK_SIZE as f32).floor() as i32,
    );
    let is_chunk_loaded = chunk_manager.chunks_gerados.contains(&chunk_futuro);

    let forward = transform.forward().normalize_or_zero();
    let right = transform.right().normalize_or_zero();
    let velocidade_andar = if under_gravity { 8.0 } else { 2.0 };
    let mut dir = Vec3::ZERO;

    if is_chunk_loaded {
        if input.pressed(KeyCode::KeyW) { dir += forward; }
        if input.pressed(KeyCode::KeyS) { dir -= forward; }
        if input.pressed(KeyCode::KeyA) { dir -= right; }
        if input.pressed(KeyCode::KeyD) { dir += right; }
    } else {
        player.velocidade_y = 0.0;
    }

    let mut move_delta = dir.normalize_or_zero() * velocidade_andar * dt;
    // Anula a velocidade vertical FÍSICA para ele deslizar perfeitamente no plano
    move_delta -= move_delta.dot(physics_up) * physics_up;
    transform.translation += move_delta;

    if is_chunk_loaded {
        if under_gravity {
            if player.no_chao && player.velocidade_y <= 0.0 {
                player.velocidade_y = -0.5; 
            } else {
                player.velocidade_y -= 25.0 * dt;
            }
        } else {
            player.velocidade_y = player.velocidade_y.lerp(0.0, dt * 2.0);
        }

        player.velocidade_y = player.velocidade_y.clamp(-20.0, 20.0);
        
        let mut nova_pos = transform.translation;
        nova_pos += physics_up * player.velocidade_y * dt;
        
        // Minkowski varre usando o CIMA físico do cubo
        let tocou_no_chao = crate::physics::resolver_colisao_minkowski(&mundo.mapa, &mut nova_pos, physics_up);

        if tocou_no_chao {
            if player.velocidade_y < 0.0 {
                player.no_chao = true;
                player.velocidade_y = 0.0;
            }
        } else {
            player.no_chao = false;
        }
        
        transform.translation = nova_pos;

        // Pular também empurra no eixo Cúbico perfeito
        if input.pressed(KeyCode::Space) && under_gravity && player.no_chao {
            player.velocidade_y = 10.0;
            player.no_chao = false;
        }
    }
}


--- Caminho do Arquivo: src/player/mod.rs ---
// src/player/mod.rs
use bevy::prelude::*;
use crate::world::PLANET_RADIUS;

pub mod movement;
pub mod god_mode; // <--- Registrando o novo módulo!

pub const GRAVITY_INFLUENCE_RADIUS: f32 = PLANET_RADIUS * 5.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, movement::spawn_player)
           .add_systems(Update, (
               movement::tratar_inputs_estado,
               movement::rotacionar_camera,
               // Chama a função do arquivo novo quando estiver no modo Deus:
               god_mode::movimento_god_mode.run_if(movement::is_god_mode),
               movement::movimento_sobrevivencia.run_if(movement::is_survival_mode),
           ));
    }
}

#[derive(Component)]
pub struct Player {
    pub velocidade_y: f32,
    pub no_chao: bool,
    pub pitch: f32,
    #[allow(dead_code)]
    pub yaw: f32,
    pub god_mode: bool,
    pub god_speed: f32,
}


--- Caminho do Arquivo: src/player/god_mode.rs ---
use bevy::prelude::*;
use super::Player;
use crate::camera::MainCamera;

pub fn movimento_god_mode(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    // Simplifiquei a query para pegar os dois componentes de uma vez
    mut query_player: Query<(&mut Transform, &Player)>, 
    query_camera: Query<&GlobalTransform, With<MainCamera>>,
) {
    let Ok((mut transform, player)) = query_player.get_single_mut() else { return; };
    let Ok(camera_global) = query_camera.get_single() else { return; };

    // ----------------------------------------------------------------
    // 1. ORIENTAÇÃO ESFÉRICA (Para não inverter os eixos do mouse!)
    // ----------------------------------------------------------------
    let pos_atual = transform.translation;
    let planet_up = pos_atual.normalize_or_zero();
    
    if planet_up != Vec3::ZERO {
        let fwd: Vec3 = transform.forward().into();
        let mut proj_fwd = (fwd - fwd.dot(planet_up) * planet_up).normalize_or_zero();
        if proj_fwd == Vec3::ZERO { proj_fwd = transform.up().into(); }

        if let (Ok(dir_fwd), Ok(dir_up)) = (Dir3::new(proj_fwd), Dir3::new(planet_up)) {
            let target_rotation = Transform::default().looking_to(dir_fwd, dir_up).rotation;
            // O God Mode agora sabe para onde fica o "chão" e gira o seu corpo suavemente
            transform.rotation = transform.rotation.slerp(target_rotation, time.delta_seconds() * 10.0);
        }
    }

    // ----------------------------------------------------------------
    // 2. MOVIMENTO LIVRE DE DRONE (6 Graus de Liberdade)
    // ----------------------------------------------------------------
    let forward = camera_global.forward().normalize_or_zero();
    let right = camera_global.right().normalize_or_zero();
    let up = camera_global.up().normalize_or_zero();

    let mut dir = Vec3::ZERO;

    if input.pressed(KeyCode::KeyW) { dir += forward; }
    if input.pressed(KeyCode::KeyS) { dir -= forward; }
    if input.pressed(KeyCode::KeyA) { dir -= right; }
    if input.pressed(KeyCode::KeyD) { dir += right; }
    
    // Q e E respeitam a visão da câmera
    if input.pressed(KeyCode::KeyQ) { dir -= up; }
    if input.pressed(KeyCode::KeyE) { dir += up; }

    transform.translation += dir.normalize_or_zero() * player.god_speed * time.delta_seconds();
}


--- Caminho do Arquivo: src/player/camera.rs ---
use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use super::Player;

pub fn rotacionar_camera(
    mut mouse_motion_events: EventReader<MouseMotion>, 
    mut query: Query<(&mut Transform, &mut Player)>,
) {
    let Ok((mut transform, mut player)) = query.get_single_mut() else { return };
    let mut mouse_dx = 0.0; 
    let mut mouse_dy = 0.0;
    
    for ev in mouse_motion_events.read() { 
        mouse_dx -= ev.delta.x * 0.003; 
        mouse_dy -= ev.delta.y * 0.003; 
    }
    
    player.pitch = (player.pitch + mouse_dy).clamp(-1.5, 1.5);
    
    if player.god_mode {
        player.yaw += mouse_dx;
        transform.rotation = Quat::from_rotation_y(player.yaw) * Quat::from_rotation_x(player.pitch);
    } else { 
        transform.rotate_local_y(mouse_dx); 
    }
}


--- Caminho do Arquivo: src/physics/mod.rs ---
// src/physics/mod.rs
use crate::world::TipoBloco;
use bevy::prelude::*;
use bevy::utils::HashMap;

// Algoritmo Minkowski Sweep (Esfera deslizando em AABBs)
pub fn resolver_colisao_minkowski(
    mapa: &HashMap<IVec3, TipoBloco>,
    pos: &mut Vec3,
    up: Vec3,
) -> bool {
    let radius = 0.35; // Raio da cápsula
    let altura = 1.0; // Distância entre a esfera base (pé) e a esfera topo (cabeça)
    let mut tocou_no_chao = false;

    let r = 2; // Área de busca
    let cx = pos.x.round() as i32;
    let cy = pos.y.round() as i32;
    let cz = pos.z.round() as i32;

    // Resolve colisões iterativamente para deslizar suavemente pelas quinas
    for _ in 0..3 {
        for x in -r..=r {
            for y in -r..=r {
                for z in -r..=r {
                    let b_pos = IVec3::new(cx + x, cy + y, cz + z);
                    if mapa.contains_key(&b_pos) {
                        let v_min = b_pos.as_vec3() - Vec3::splat(0.5);
                        let v_max = b_pos.as_vec3() + Vec3::splat(0.5);

                        // ====================================================
                        // 1. ESFERA DA BASE (PÉS)
                        // ====================================================
                        let p_base = *pos + up * radius;
                        let closest_base = p_base.clamp(v_min, v_max);
                        let dist_base = p_base.distance(closest_base);

                        if dist_base < radius {
                            let raw_push_dir = (p_base - closest_base).normalize_or_zero();

                            if raw_push_dir == Vec3::ZERO {
                                // O centro do jogador está DENTRO do bloco. 
                                // Verifica a direção real entre o centro do bloco e o jogador
                                let center_dir = (p_base - b_pos.as_vec3()).normalize_or_zero();
                                let dot_up = center_dir.dot(up);
                                
                                if dot_up > 0.5 { 
                                    // Chão verdadeiro: Ejeta pra cima
                                    *pos += up * 0.05;
                                    tocou_no_chao = true;
                                } else { 
                                    // Parede: Ejeta estritamente pro lado
                                    let push_side = center_dir - dot_up * up;
                                    *pos += push_side.normalize_or_zero() * 0.05;
                                }
                            } else {
                                let dot_up = raw_push_dir.dot(up);
                                let mut final_push = raw_push_dir;

                                // Tolerância extrema para não subir em paredes
                                let is_wall = dot_up.abs() < 0.98;

                                if is_wall {
                                    final_push -= dot_up * up;
                                    final_push = final_push.normalize_or_zero();
                                }

                                *pos += final_push * (radius - dist_base);

                                if dot_up > 0.85 {
                                    tocou_no_chao = true;
                                }
                            }
                        }

                        // ====================================================
                        // 2. ESFERA DO TOPO (CABEÇA)
                        // ====================================================
                        let p_topo = *pos + up * altura;
                        let closest_topo = p_topo.clamp(v_min, v_max);
                        let dist_topo = p_topo.distance(closest_topo);

                        if dist_topo < radius {
                            let raw_push_dir = (p_topo - closest_topo).normalize_or_zero();
                            
                            if raw_push_dir == Vec3::ZERO {
                                let center_dir = (p_topo - b_pos.as_vec3()).normalize_or_zero();
                                let dot_up = center_dir.dot(up);
                                
                                if dot_up < -0.5 { 
                                    // Bateu a cabeça no teto puro: Empurra pra baixo
                                    *pos -= up * 0.05; 
                                } else { 
                                    // Parede na altura do ombro/cabeça: Empurra pro lado
                                    let push_side = center_dir - dot_up * up;
                                    *pos += push_side.normalize_or_zero() * 0.05;
                                }
                            } else {
                                let dot_up = raw_push_dir.dot(up);
                                let mut final_push = raw_push_dir;

                                // A Cabeça AGORA TAMBÉM é impedida de agarrar na parede!
                                let is_wall = dot_up.abs() < 0.98;

                                if is_wall {
                                    final_push -= dot_up * up;
                                    final_push = final_push.normalize_or_zero();
                                }

                                *pos += final_push * (radius - dist_topo);
                            }
                        }
                    }
                }
            }
        }
    }
    tocou_no_chao
}


--- Caminho do Arquivo: src/world/mod.rs ---
// src/world/mod.rs
use bevy::prelude::*;
use bevy::tasks::Task;
use bevy::utils::{HashMap, HashSet};

mod generator;
mod mesher;

pub const PLANET_RADIUS: f32 = 80.0; 
pub const CHUNK_SIZE: i32 = 32;
pub const RENDER_DISTANCE: i32 = 2;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum TipoBloco { Grama, Pedra, Areia, Agua, Nucleo, Neve }

#[derive(Resource, Default)]
pub struct VoxelWorld {
    pub mapa: HashMap<IVec3, TipoBloco>,
}

#[derive(Resource, Default)]
pub struct ChunkManager {
    pub chunks_gerados: HashSet<IVec3>,
    pub meshes_ativos: HashMap<IVec3, Vec<Entity>>,
    pub chunks_para_remesh: HashSet<IVec3>, 
    // NOVO: Guarda as tarefas de multithreading que estão rodando na CPU no momento
    pub tarefas_geracao: HashMap<IVec3, Task<Vec<(IVec3, TipoBloco)>>>,
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<VoxelWorld>()
           .init_resource::<ChunkManager>()
           .add_systems(Update, generator::gerenciar_chunks); 
    }
}


--- Caminho do Arquivo: src/world/generator.rs ---
// src/world/generator.rs
use super::mesher::construir_mesh_do_chunk;
use super::{CHUNK_SIZE, ChunkManager, PLANET_RADIUS, RENDER_DISTANCE, TipoBloco, VoxelWorld};
use crate::player::Player;
use bevy::prelude::*;
use bevy::tasks::AsyncComputeTaskPool;
use futures_lite::future;
use noise::{NoiseFn, OpenSimplex};

pub fn calcular_bloco(x: f32, y: f32, z: f32, simplex: &OpenSimplex) -> Option<TipoBloco> {
    let pos = Vec3::new(x, y, z);
    let dist_base = (pos.x.powi(4) + pos.y.powi(4) + pos.z.powi(4)).powf(0.25);

    if dist_base > PLANET_RADIUS + 40.0 {
        return None;
    }

    let dir = pos.normalize_or_zero();
    let (nx, ny, nz) = (dir.x as f64, dir.y as f64, dir.z as f64);

    let base_altura = simplex.get([nx * 1.5, ny * 1.5, nz * 1.5]) * 22.0;
    let mut modificador_relevo = base_altura;

    if base_altura > 0.0 {
        modificador_relevo += simplex.get([nx * 4.0, ny * 4.0, nz * 4.0]) * 8.0;
        if base_altura > 5.0 {
            modificador_relevo +=
                (simplex.get([nx * 8.0, ny * 8.0, nz * 8.0]).abs() * -1.0 + 0.5) * 25.0;
        }
    }

    modificador_relevo = (modificador_relevo / 2.0).round() * 2.0;

    let superficie = PLANET_RADIUS + modificador_relevo as f32;
    let nivel_mar = PLANET_RADIUS + 0.5;

    if dist_base <= superficie {
        let altitude = dist_base - PLANET_RADIUS;
        if altitude > 22.0 {
            return Some(TipoBloco::Neve);
        }
        if altitude > 8.0 {
            return Some(TipoBloco::Pedra);
        }
        if dist_base <= nivel_mar + 1.5 && modificador_relevo < 2.0 {
            return Some(TipoBloco::Areia);
        }
        if dist_base > superficie - 4.0 {
            return Some(TipoBloco::Grama);
        }
        return Some(TipoBloco::Nucleo);
    } else if dist_base <= nivel_mar {
        return Some(TipoBloco::Agua);
    }
    None
}

pub fn gerenciar_chunks(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    mut mundo: ResMut<VoxelWorld>,
    mut chunk_manager: ResMut<ChunkManager>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };
    let p_pos = player_transform.translation;

    let player_chunk = IVec3::new(
        (p_pos.x / CHUNK_SIZE as f32).floor() as i32,
        (p_pos.y / CHUNK_SIZE as f32).floor() as i32,
        (p_pos.z / CHUNK_SIZE as f32).floor() as i32,
    );

    let mut chunks_na_area = Vec::new();

    // Forma de Cubo (Mais estável que esfera para o Greedy Mesher)
    for cx in -RENDER_DISTANCE..=RENDER_DISTANCE {
        for cy in -RENDER_DISTANCE..=RENDER_DISTANCE {
            for cz in -RENDER_DISTANCE..=RENDER_DISTANCE {
                chunks_na_area.push(player_chunk + IVec3::new(cx, cy, cz));
            }
        }
    }

    chunks_na_area.sort_by_key(|c| {
        let dx = c.x - player_chunk.x;
        let dy = c.y - player_chunk.y;
        let dz = c.z - player_chunk.z;
        dx * dx + dy * dy + dz * dz
    });

    // 1. DESCARREGAR LIXO VISUAL E CANCELAR TAREFAS ANTIGAS
    let mut chunks_a_remover = Vec::new();
    for chunk_pos in chunk_manager.meshes_ativos.keys() {
        if !chunks_na_area.contains(chunk_pos) {
            chunks_a_remover.push(*chunk_pos);
        }
    }
    for chunk_pos in chunks_a_remover {
        if let Some(entities) = chunk_manager.meshes_ativos.remove(&chunk_pos) {
            for entity in entities {
                commands.entity(entity).despawn();
            }
        }
        chunk_manager.chunks_para_remesh.remove(&chunk_pos);
    }

    let mut tarefas_a_remover = Vec::new();
    for chunk_pos in chunk_manager.tarefas_geracao.keys() {
        if !chunks_na_area.contains(chunk_pos) {
            tarefas_a_remover.push(*chunk_pos);
        }
    }
    for chunk_pos in tarefas_a_remover {
        chunk_manager.tarefas_geracao.remove(&chunk_pos); // Derrubar a Task poupa processamento!
    }

    // 2. DISPARAR GERAÇÃO EM BACKGROUND (MULTITHREADING)
    let thread_pool = AsyncComputeTaskPool::get();
    for chunk_pos in &chunks_na_area {
        // Se ainda não geramos esse chunk e não há uma thread trabalhando nele...
        if !chunk_manager.chunks_gerados.contains(chunk_pos)
            && !chunk_manager.tarefas_geracao.contains_key(chunk_pos)
        {
            let c_pos = *chunk_pos;

            // Joga a matemática pesada para os núcleos ociosos do seu processador!
            let task = thread_pool.spawn(async move {
                let simplex = OpenSimplex::new(42);
                let mut blocos_gerados = Vec::new();

                let start_x = c_pos.x * CHUNK_SIZE;
                let start_y = c_pos.y * CHUNK_SIZE;
                let start_z = c_pos.z * CHUNK_SIZE;

                for x in 0..CHUNK_SIZE {
                    for y in 0..CHUNK_SIZE {
                        for z in 0..CHUNK_SIZE {
                            let world_pos = IVec3::new(start_x + x, start_y + y, start_z + z);
                            if let Some(tipo) = calcular_bloco(
                                world_pos.x as f32,
                                world_pos.y as f32,
                                world_pos.z as f32,
                                &simplex,
                            ) {
                                blocos_gerados.push((world_pos, tipo));
                            }
                        }
                    }
                }
                blocos_gerados
            });

            chunk_manager.tarefas_geracao.insert(c_pos, task);
        }
    }

    // 3. COLETAR RESULTADOS SEM TRAVAR O JOGO
    let mut chunks_concluidos = Vec::new();
    for (chunk_pos, task) in chunk_manager.tarefas_geracao.iter_mut() {
        if let Some(blocos) = future::block_on(future::poll_once(task)) {
            chunks_concluidos.push((*chunk_pos, blocos));
            break; // <--- O SEGREDO 1: Só injeta 1 chunk no HashMap por frame!
        }
    }

    // Salvar os resultados prontos na RAM (Extremamente rápido, O(1))
    for (chunk_pos, blocos) in chunks_concluidos {
        chunk_manager.tarefas_geracao.remove(&chunk_pos);
        for (w_pos, tipo) in blocos {
            mundo.mapa.insert(w_pos, tipo);
        }
        chunk_manager.chunks_gerados.insert(chunk_pos);
        chunk_manager.chunks_para_remesh.insert(chunk_pos);

        // Avisa os vizinhos que ganhamos blocos novos (Remove paredes internas de vidro)
        for dir in [
            IVec3::X,
            IVec3::NEG_X,
            IVec3::Y,
            IVec3::NEG_Y,
            IVec3::Z,
            IVec3::NEG_Z,
        ] {
            let viz = chunk_pos + dir;
            if chunk_manager.chunks_gerados.contains(&viz) {
                chunk_manager.chunks_para_remesh.insert(viz);
            }
        }
    }

    // 4. A CURA DO CACHE FANTASMA:
    for chunk_pos in &chunks_na_area {
        if chunk_manager.chunks_gerados.contains(chunk_pos)
            && !chunk_manager.meshes_ativos.contains_key(chunk_pos)
        {
            chunk_manager.chunks_para_remesh.insert(*chunk_pos);
        }
    }

    // 5. DESENHAR POLÍGONOS (1 por frame para segurar o FPS)
    let mut chunk_to_mesh = None;
    for c in &chunks_na_area {
        if chunk_manager.chunks_para_remesh.contains(c) {
            chunk_to_mesh = Some(*c);
            break;
        }
    }

    if let Some(chunk_pos) = chunk_to_mesh {
        chunk_manager.chunks_para_remesh.remove(&chunk_pos);
        if let Some(entities) = chunk_manager.meshes_ativos.remove(&chunk_pos) {
            for entity in entities {
                commands.entity(entity).despawn();
            }
        }
        let novas_entidades = construir_mesh_do_chunk(
            chunk_pos,
            &mundo,
            &mut commands,
            &mut meshes,
            &mut materials,
        );
        chunk_manager
            .meshes_ativos
            .insert(chunk_pos, novas_entidades);
    }
}



--- Caminho do Arquivo: src/world/mesher.rs ---
// src/world/mesher.rs
use super::{CHUNK_SIZE, TipoBloco, VoxelWorld};
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;

fn cor_do_bloco(tipo: TipoBloco) -> [f32; 4] {
    match tipo {
        TipoBloco::Grama => [0.2, 0.7, 0.2, 1.0],
        TipoBloco::Pedra => [0.4, 0.4, 0.45, 1.0],
        TipoBloco::Areia => [0.9, 0.8, 0.5, 1.0],
        TipoBloco::Nucleo => [0.2, 0.2, 0.2, 1.0],
        TipoBloco::Agua => [0.1, 0.4, 0.8, 0.6],
        TipoBloco::Neve => [0.95, 0.95, 1.0, 1.0],
    }
}

fn is_transparent(tipo: TipoBloco) -> bool {
    tipo == TipoBloco::Agua
}

#[derive(Default)]
struct ChunkMeshBuilder {
    positions: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    colors: Vec<[f32; 4]>,
    indices: Vec<u32>,
}

impl ChunkMeshBuilder {
    fn add_quad(
        &mut self,
        v0: [f32; 3],
        v1: [f32; 3],
        v2: [f32; 3],
        v3: [f32; 3],
        n: [f32; 3],
        c: [f32; 4],
        reverse: bool,
    ) {
        let base = self.positions.len() as u32;
        self.positions.extend([v0, v1, v2, v3]);
        self.normals.extend([n, n, n, n]);
        self.colors.extend([c, c, c, c]);
        if reverse {
            self.indices
                .extend([base, base + 2, base + 1, base, base + 3, base + 2]);
        } else {
            self.indices
                .extend([base, base + 1, base + 2, base, base + 2, base + 3]);
        }
    }
    fn is_empty(&self) -> bool {
        self.positions.is_empty()
    }
    fn build_mesh(self) -> Mesh {
        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        );
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, self.positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, self.normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, self.colors);
        mesh.insert_indices(Indices::U32(self.indices));
        mesh
    }
}

pub fn construir_mesh_do_chunk(
    chunk_pos: IVec3,
    mundo: &VoxelWorld,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> Vec<Entity> {
// Voltamos ao padrão da indústria: as costas invisíveis dos polígonos não são renderizadas!
    let mat_opaque = materials.add(StandardMaterial { 
        base_color: Color::WHITE, 
        alpha_mode: AlphaMode::Opaque, 
        perceptual_roughness: 0.9, 
        ..default() 
    });
    
    let mat_transparent = materials.add(StandardMaterial { 
        base_color: Color::WHITE, 
        alpha_mode: AlphaMode::Blend, 
        perceptual_roughness: 0.1, 
        ..default() 
    });

    let cx = chunk_pos.x * CHUNK_SIZE;
    let cy = chunk_pos.y * CHUNK_SIZE;
    let cz = chunk_pos.z * CHUNK_SIZE;

    // =========================================================================
    // O SEGREDO 2: CACHE LINEAR 1D (Derruba o tempo de Meshing de 40ms para 2ms)
    // =========================================================================
    let p_size = CHUNK_SIZE + 2; // +2 para incluir a "casca" de vizinhos
    let mut cache_local = vec![None; (p_size * p_size * p_size) as usize];
    let start_x = cx - 1;
    let start_y = cy - 1;
    let start_z = cz - 1;

    // Fazemos apenas ~39k chamadas ao HashMap UMA VEZ para preencher o Cache
    for x in 0..p_size {
        for y in 0..p_size {
            for z in 0..p_size {
                let w_pos = IVec3::new(start_x + x, start_y + y, start_z + z);
                let idx = (z * p_size * p_size) + (y * p_size) + x;
                cache_local[idx as usize] = mundo.mapa.get(&w_pos).copied();
            }
        }
    }

    // Função interna super-rápida (Array indexing) para substituir o HashMap
    let get_cached = |w_pos: IVec3| -> Option<TipoBloco> {
        let lx = w_pos.x - start_x;
        let ly = w_pos.y - start_y;
        let lz = w_pos.z - start_z;
        let idx = (lz * p_size * p_size) + (ly * p_size) + lx;
        cache_local[idx as usize]
    };
    // =========================================================================

    let dirs = [
        (0, 1, IVec3::X, [1.0, 0.0, 0.0]),
        (0, -1, IVec3::NEG_X, [-1.0, 0.0, 0.0]),
        (1, 1, IVec3::Y, [0.0, 1.0, 0.0]),
        (1, -1, IVec3::NEG_Y, [0.0, -1.0, 0.0]),
        (2, 1, IVec3::Z, [0.0, 0.0, 1.0]),
        (2, -1, IVec3::NEG_Z, [0.0, 0.0, -1.0]),
    ];

    let mut b_opaque = ChunkMeshBuilder::default();
    let mut b_transp = ChunkMeshBuilder::default();

    for &(d, sign, dir_vec, normal) in &dirs {
        let u = (d + 1) % 3;
        let v = (d + 2) % 3;
        for slice in 0..CHUNK_SIZE {
            let mut mask = vec![None; (CHUNK_SIZE * CHUNK_SIZE) as usize];
            for j in 0..CHUNK_SIZE {
                for i in 0..CHUNK_SIZE {
                    let mut pos = IVec3::ZERO;
                    pos[d] = slice;
                    pos[u] = i;
                    pos[v] = j;
                    let world_pos = IVec3::new(cx, cy, cz) + pos;

                    // Lendo do nosso Cache ultrarrápido ao invés do HashMap!
                    let b_current = get_cached(world_pos);
                    let b_neighbor = get_cached(world_pos + dir_vec);

                    if let Some(t_curr) = b_current {
                        let should_draw = if is_transparent(t_curr) {
                            b_neighbor.is_none()
                        } else {
                            b_neighbor.map_or(true, is_transparent)
                        };
                        if should_draw {
                            mask[(j * CHUNK_SIZE + i) as usize] = Some(t_curr);
                        }
                    }
                }
            }

            let mut j = 0;
            while j < CHUNK_SIZE {
                let mut i = 0;
                while i < CHUNK_SIZE {
                    if let Some(tipo) = mask[(j * CHUNK_SIZE + i) as usize] {
                        let mut width = 1;
                        while i + width < CHUNK_SIZE
                            && mask[(j * CHUNK_SIZE + i + width) as usize] == Some(tipo)
                        {
                            width += 1;
                        }
                        let mut height = 1;
                        'outer: while j + height < CHUNK_SIZE {
                            for w in 0..width {
                                if mask[((j + height) * CHUNK_SIZE + i + w) as usize] != Some(tipo)
                                {
                                    break 'outer;
                                }
                            }
                            height += 1;
                        }
                        for h in 0..height {
                            for w in 0..width {
                                mask[((j + h) * CHUNK_SIZE + i + w) as usize] = None;
                            }
                        }

                        let color = cor_do_bloco(tipo);
                        let offset_d = if sign == 1 { 0.5 } else { -0.5 };
                        let mut p0 = [0.0; 3];
                        let mut p1 = [0.0; 3];
                        let mut p2 = [0.0; 3];
                        let mut p3 = [0.0; 3];
                        let bases = [cx as f32, cy as f32, cz as f32];
                        p0[d] = bases[d] + slice as f32 + offset_d;
                        p1[d] = p0[d];
                        p2[d] = p0[d];
                        p3[d] = p0[d];
                        p0[u] = bases[u] + i as f32 - 0.5;
                        p0[v] = bases[v] + j as f32 - 0.5;
                        p1[u] = bases[u] + (i + width) as f32 - 0.5;
                        p1[v] = bases[v] + j as f32 - 0.5;
                        p2[u] = bases[u] + (i + width) as f32 - 0.5;
                        p2[v] = bases[v] + (j + height) as f32 - 0.5;
                        p3[u] = bases[u] + i as f32 - 0.5;
                        p3[v] = bases[v] + (j + height) as f32 - 0.5;

                        let reverse_winding = sign == -1;
                        if is_transparent(tipo) {
                            b_transp.add_quad(p0, p1, p2, p3, normal, color, reverse_winding);
                        } else {
                            b_opaque.add_quad(p0, p1, p2, p3, normal, color, reverse_winding);
                        }
                        i += width;
                    } else {
                        i += 1;
                    }
                }
                j += 1;
            }
        }
    }

    let mut spawnadas = Vec::new();
    if !b_opaque.is_empty() {
        spawnadas.push(
            commands
                .spawn(PbrBundle {
                    mesh: meshes.add(b_opaque.build_mesh()),
                    material: mat_opaque,
                    ..default()
                })
                .id(),
        );
    }
    if !b_transp.is_empty() {
        spawnadas.push(
            commands
                .spawn(PbrBundle {
                    mesh: meshes.add(b_transp.build_mesh()),
                    material: mat_transparent,
                    ..default()
                })
                .id(),
        );
    }
    spawnadas
}



--- Caminho do Arquivo: src/main.rs ---
// src/main.rs
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PresentMode, WindowResolution}; // <-- ATUALIZE OS IMPORTS


// Declarando nossos módulos isolados
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
                present_mode: PresentMode::AutoNoVsync, // <-- DESTRAVA O LIMITADOR DE FPS DA TELA
                ..default()
            }),
            ..default()
        }))
        .add_plugins(FrameTimeDiagnosticsPlugin)
        
        // --- NOSSOS PLUGINS DA ENGINE ---
        .add_plugins((
            camera::CameraPlugin,
            hud::HudPlugin,
            world::WorldPlugin,
            player::PlayerPlugin,
        ))

        .insert_resource(ClearColor(Color::srgb(0.4, 0.7, 0.9)))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 100.0,
        })
        .add_systems(Update, gerenciar_cursor)
        .run();
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


