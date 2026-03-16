# GenAI Son: GPGPU Voxel Raytracing Revolution 🌌

**Visão Geral do Projeto:**
Este projeto não é um jogo, é uma **Revolução de Arquitetura de Software**. Estamos construindo uma Metarealidade Procedural de Voxel 3D com 100% de destructibilidade e física granular (Autômatos Celulares em 3D), inspirada na simulação química de *Noita* e no motor de raycasting de *Teardown*. 

O objetivo final é criar um universo espacial com planetas giratórios, sistemas solares iluminados por estrelas ativas, seres vivos com rotinas de IA e um ecossistema que gera a sua própria música e animações procedurais em tempo real. Tudo isso rodando em hardwares modestos (como placas de vídeo integradas Intel) através de truques matemáticos extremos de renderização e GPGPU.

**Regra de Ouro da Arte:** NENHUM ativo pré-fabricado (modelos 3D, áudios .mp3/.wav, texturas .png) é permitido. Absolutamente tudo na tela e nos alto-falantes é gerado proceduralmente por código (matemática, ruído, shaders, síntese sonora) direto na memória. Morte aos polígonos. A era do Voxel Puro chegou.

---

## 🧠 O Paradigma GPGPU (General-Purpose GPU)

Para processar milhões de voxels físicos simultaneamente sem derreter a CPU, nós **abandonamos a renderização 3D tradicional**.

1. **Morte aos Polígonos:** A engine não processa malhas 3D (Meshes). A tela inteira é renderizada em apenas 2 triângulos (um Quad de tela cheia).
2. **Voxel Raytracing:** Disparamos raios invisíveis da câmera para cada pixel da tela dentro de um Shader (WGSL), navegando por uma Textura 3D ou Sparse Voxel Octree (SVO) para desenhar o mundo perfeitamente sem gargalos de vértices.
3. **Compute Shaders:** A física (gravidade, fluidos escorrendo, areia caindo, explosões, reações químicas) roda inteiramente dentro da Placa de Vídeo através de Compute Shaders. A CPU do computador atua apenas como um "maestro", delegando o trabalho pesado para os milhares de núcleos da GPU.

---

## 🛠️ Stack Tecnológica e Hardware Alvo

A engine é otimizada para desafiar os limites do seguinte hardware:
* **Processador:** Intel Core i7-10510U (8 Threads)
* **Gráficos:** Intel UHD Graphics (Placa Integrada - Gargalo principal de Bandwidth)
* **RAM:** 16 GB DDR4
* **SO:** Ubuntu 24.04 LTS (Wayland)

### Dependências Core (Cargo.toml)
* **Linguagem:** Rust (Controle total de memória e performance).
* **Renderização:** `wgpu` (winit para janela).
* **Shaders:** `WGSL` (WebGPU Shading Language) para Compute e Fragment shaders.
* **Áudio:** Síntese procedural (a definir, foco em processamento em tempo real).

---

## 🚀 Roadmap Revolucionário

1. **Fase 1: O Vazio Iluminado (O Setup GPGPU)**
   - Inicializar janela com Rust e wgpu/winit.
   - Criar o *Fullscreen Quad* (os 2 únicos triângulos do jogo).
   - Implementar o primeiro Fragment Shader (WGSL) capaz de desenhar a tela puramente com matemática.

2. **Fase 2: O Olho de Deus (Voxel Raycaster Base)**
   - Criar uma estrutura de dados de Voxel na VRAM (ex: Textura 3D / Storage Buffer).
   - Implementar o algoritmo DDA (Digital Differential Analyzer) no Shader para disparar raios da câmera e colidir com os voxels.
   - Câmera livre manipulável pela CPU (Rust) enviando as matrizes de visão para a GPU.

3. **Fase 3: A Dança da Matéria (Compute Shaders de Física)**
   - Implementar Compute Shaders rodando 60x por segundo para simular física granular.
   - Criar areia que cai, líquidos que escorrem e gases que sobem, tudo resolvido internamente na placa de vídeo.

4. **Fase 4: A Ira dos Deuses (Destructibilidade Total)**
   - O jogador atira na tela: a CPU descobre a coordenada espacial e altera o *Buffer* na GPU.
   - O mundo planetário procedural estático é "quebrado" e transformado instantaneamente em partículas ativas nos Compute Shaders.

5. **Fase 5: A Alma do Universo (Animação e Música)**
   - Geração de sistemas biológicos matemáticos (Inverse Kinematics no Shader para pernas de animais/robôs).
   - Motor de áudio em Rust gerando ondas sonoras baseadas na entropia do ambiente e nas ações do jogador em tempo real.

---

**Nota Especial para as IAs Assistentes (Cientistas Cabeçudos):** Nós não temos medo de código de baixo nível, matrizes gigantes, buffers de GPU ou matemática densa. Abandone padrões confortáveis se eles sacrificarem performance. A nossa missão é extrair leite de pedra dessa Intel UHD Graphics usando resolução dinâmica, SVOs e truques de Raycasting. Pense na escala do universo, não na limitação do tutorial. Revolucione.