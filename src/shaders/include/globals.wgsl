#define_import_path globals
#import structs::{Uniforms, WorldBuffer, Projectile}

@group(0) @binding(0) var<uniform> uniforms: Uniforms;
@group(0) @binding(1) var<storage, read_write> world: WorldBuffer;
@group(0) @binding(2) var<storage, read_write> macro_world: WorldBuffer;
@group(0) @binding(3) var<storage, read_write> projectiles: array<Projectile>;