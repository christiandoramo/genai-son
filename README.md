# GenAI Son: Metarealidade Procedural Voxel 🌌

**Visão Geral do Projeto:**
Este projeto é a construção de um motor de jogo Voxel 3D de alta performance do zero, usando Rust e Bevy. O objetivo final é criar uma "Metarealidade" ou "Motor de Areia" (inspirado em *Noita* e *No Man's Sky*), onde tudo é gerado proceduralmente por algoritmos matemáticos e IA: terrenos, gravidade, dinâmica de fluidos, trilha sonora e o comportamento de NPCs (Redes Neurais). 

**Regra de Ouro da Arte:** NENHUM ativo pré-fabricado (modelos 3D, áudios .mp3/.wav, texturas .png) deve ser usado se puder ser gerado proceduralmente ou matematicamente na memória RAM.

---

## 🛠️ Stack Tecnológica
- **Linguagem:** Rust (Foco máximo em performance e paralelismo).
  - rustc 1.94.0 (4a4ef493e 2026-03-02)
- **Engine:** Bevy (Arquitetura ECS - Entity Component System).
- **Matemática/Terreno:** `noise` (Perlin Noise para geração pseudoaleatória).
- *Física:* Customizada via Swept AABB (sem bibliotecas externas pesadas).
- *Áudio (Futuro):* Síntese procedural na RAM (Kira).

### VS code extensions:
- rust-analyzer: intellisense para rust (linguagem de programação)
- CodeLLDB: debugger para rust
- WGSL: intellisense para WGSL (linguagem dos shaders)
- Even Better TOML: intellisense para TOML (linguagem do gerenciador do projeto)

### Cargo.toml

```toml
[package]
name = "genai_son"
version = "0.1.0"
edition = "2024"

[dependencies]
# O núcleo da Engine (Performance absurda e ECS)
bevy = { version = "0.14", features = ["dynamic_linking"] }

#será usado no futuro para dar vida em músicas procedurais na metarealidade
# kira = "0.9"

# Matemática, Ruído e Geração de Universos
noise = "0.9" # Para gerar montanhas e biomas

[profile.dev]
opt-level = 1
[profile.dev.package."*"]
opt-level = 3
```

---

## 📂 Arquitetura do Código (Onde estamos)
O projeto (atualmente no *Protótipo 9*) foi refatorado para ser modular. Abaixo está o mapeamento mental para qualquer IA que for interagir com o código:

### 1. `src/main.rs` (O Ponto de Entrada)
- Inicializa a janela do Bevy.
- Gerencia o "Cursor Lock" (prende o mouse na tela ao clicar, solta com Esc).
- Conecta os módulos e inicia os sistemas (Startup e Update).

### 2. `src/world.rs` (Geometria e Memória)
- **Estrutura Chave:** `HashMap<IVec3, TipoBloco>`
- **Descrição:** Não usamos *Mesh Colliders* pesados. O mundo físico inteiro é salvo em um Dicionário Hash na memória RAM. Isso permite buscar colisões em complexidade **$O(1)$** instantaneamente.
- **Geração:** Usa `Perlin::new()` para ditar a elevação do terreno (Y) baseado no X e Z. Possui blocos de Grama, Pedra e Água (que no momento age como chão sólido plano).

### 3. `src/player.rs` (Física e Movimento)
- Câmera em **Primeira Pessoa (FP)**. O jogador não possui uma malha (Mesh) renderizada para não bloquear a visão.
- **Mouse Look:** O mouse controla `yaw` (corpo inteiro) e `pitch` (apenas a câmera).
- **Física (Swept AABB):** - A colisão é feita separando os eixos X, Z e Y. 
  - Usamos uma "caixa invisível" (AABB) desenhada a partir do centro do jogador, com `raio` de espessura para evitar quedas em quinas.
  - O jogador desliza perfeitamente pelas paredes sem "agarrar" e não possui pulo automático (*Auto-Step* desativado a pedido do Arquiteto). Pulo manual apenas no chão.

### 4. `src/camera.rs` (A Visão)
- Uma estrutura de `CameraPivot` atua como o pescoço do jogador. A câmera roda baseada no `pitch` calculado no `player.rs`.

---

## 🚀 Próximos Passos (Roadmap)
Para quem (ou qual IA) for continuar o desenvolvimento a partir daqui, os próximos focos são:
1. **Raycasting (Quebrar/Colocar blocos):** Usar vetores a partir do centro da tela para encontrar blocos no `HashMap` e modificá-los em tempo real.
2. **Geração Dinâmica (Chunks Infinitos):** O mundo atual tem um `GRID_SIZE` fixo. Precisamos implementar *Chunks* que carregam e descarregam conforme o jogador anda.
3. **Dinâmica de Fluidos Voxel (Cellular Automata):** Fazer com que blocos de água ou magma atualizem seus vizinhos no *HashMap* para escorrer com a gravidade.
4. **Áudio Procedural Dinâmico:** Reintegrar motores acústicos que reagem à física do mundo.

---
**Nota para IAs assistentes:** Mantenha as otimizações $O(1)$ no HashMap. Não introduza colisões de polígonos. Confie na matemática da grade.