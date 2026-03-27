use std::collections::HashSet;
use std::fs;

pub struct PipelineBuilder;

impl PipelineBuilder {
    pub fn create_compute_pipeline(
        device: &wgpu::Device,
        layout: &wgpu::PipelineLayout,
        module: &wgpu::ShaderModule,
        entry: &str,
    ) -> wgpu::ComputePipeline {
        device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Compute Pipeline"),
            layout: Some(layout),
            module,
            entry_point: Some(entry),
            compilation_options: Default::default(),
            cache: None,
        })
    }
}

// O NOSSO PRÉ-PROCESSADOR BARE-METAL
pub fn load_shader_with_includes(base_dir: &str, file_path: &str) -> String {
    let mut included = HashSet::new();
    process_includes(base_dir, file_path, &mut included)
}

fn process_includes(base_dir: &str, file_path: &str, included: &mut HashSet<String>) -> String {
    // Garante que o mesmo arquivo não seja importado duas vezes (#pragma once)
    let clean_path = file_path.replace("./", "").replace("../", ""); 
    
    if !included.insert(clean_path.clone()) {
        return String::new(); // Já incluído, ignora para evitar duplicação
    }

    let full_path = format!("{}/{}", base_dir, clean_path);
    let source = fs::read_to_string(&full_path)
        .unwrap_or_else(|_| panic!("Engine Error: Shader não encontrado: {}", full_path));

    let mut final_source = String::new();
    for line in source.lines() {
        if line.trim().starts_with("#include") {
            if let Some(start) = line.find('"') {
                if let Some(end) = line[start + 1..].find('"') {
                    let include_path = &line[start + 1..start + 1 + end];
                    // Recursão! Mergulha no próximo arquivo
                    let included_source = process_includes(base_dir, include_path, included);
                    final_source.push_str(&included_source);
                    final_source.push('\n');
                    continue;
                }
            }
        }
        final_source.push_str(line);
        final_source.push('\n');
    }
    final_source
}