# GenAI Son: Metarealidade Procedural Voxel 🌌

**Visão Geral do Projeto:**
Este projeto é a construção de um motor de jogo Voxel 3D de alta performance do zero, usando Rust e Bevy. O objetivo final (não é fazer um clone de minecraft) é criar um "Metarealidade" ou "Motor de Areia" (inspirado em *Noita* e *No Man's Sky*), onde tudo é gerado proceduralmente por algoritmos matemáticos e IA: terrenos, gravidade, dinâmica de fluidos, trilha sonora e o comportamento de NPCs. 

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
# kira = "0.9" # não será usado no momento,mas o propósito é o mundo gerar músicas procedurais em tempo real com base no jogo

# Matemática, Ruído e Geração de Universos
noise = "0.9" # Para gerar montanhas e biomas

[profile.dev]
opt-level = 1
[profile.dev.package."*"]
opt-level = 3
```
---

## 🚀 Próximos Passos (Roadmap)
Para a IA que pegar o projeto agora e for continuar o desenvolvimento a partir daqui, os próximos focos são:

1. **Refatoração e melhorias:** Os códigos já estão 100% funcionais, cuidado para não modificar acidentalmente. Deve melhorar a separação de arquivos. Uma melhoria que pode fazer é colocar um raio de órbita no planeta (pode ser extremamente distante), para quando o jogador se aproximar a gravidade fazer efeito (cuidado que teram infindáveis planetas, logo deve pensar de forma que não deixe pesado, talvez transferir responsabilidade da física para o player, como já está agora?)
2. **Otimização:** Atualmente o jogo está bem lendo, 10-30 de fps e diminui conforme aumento do raio do planeta (Superelipsoide). Devem ser feitas otimizações, por exemplo algoritmo complexo chamado Greedy Meshing que funde milhares de blocos em um único "objeto" 3D. No momento nós estamos mandando a placa de vídeo desenhar e calcular a sombra de 30.000 cubos separados por frame, isso é horrível. Precisaremos acrescentar centenas de linhas de código de mesclagem para melhorar isso - as otimizações passadas foram apenas reduzir o raio do planeta, o que não faz sentido.
3. **Geração Dinâmica (Chunks Finitos-Discretos):** O mundo atual tem um `GRID_SIZE` fixo. Precisamos implementar *Chunks* que carregam e descarregam conforme o jogador anda, mas com muito cuidado, pois o mundo não é plano - isso ainda não está sendo feito. Deve ter um bom campo de visão para não causar estranhamento, para isso pode refazer a névoa para realmente deixar a renderização computacional mais econômica no meu PC. E atente-se, como é um planeta, deve renderizar corretamente conforme nos afastamos, não podem ficar pedaços faltando na vista, isso seria muito errado.
4.**Criação de um mundo voxelizada para a futura física**: permitir com que os cubos do mundo possam se dividir em voxels quando forem destruídos, mas as superfícies devem continuar 100% planas em relação a física de colisões do personagem, para ele continuar "pisando" corretamente, o mundo deve continuar em cubos como já está.
5. **Dinâmica de Fluidos Voxel (Cellular Automata):** 
Fazer duas pistolas:
- Uma pistola que atira uma bombinha explosiva ao colidir com um solo/parede e destrói onde colide, sendo deve ter efeitos semelhantes como o jogo noite (motor de areia), os pedaços do cubo devem desintegrar, e outros pedaços voarem
Fazer uma pistola que consiga atirar uma bolinha de água com gravidade atire,com que blocos de água ou magma atualizem seus vizinhos no *HashMap* para escorrer com a gravidade.
6. **Áudio Procedural Dinâmico:** Reintegrar motores acústicos que reagem à física do mundo, de forma musical, com sensação viciante e agradável.

---
**Nota para IAs assistentes:** Mantenha as otimizações $O(1)$ no HashMap. Não introduza colisões de polígonos. Confie na matemática da grade.