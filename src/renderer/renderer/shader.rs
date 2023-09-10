use std::collections::hash_map::HashMap;

use gl33::global_loader::{
    glAttachShader, glCreateProgram, glGetShaderiv, glLinkProgram, glValidateProgram, glDeleteShader,
};

enum ShaderType {
    None,
    Vertex,
    Fragment,
    Geometry,
}
struct ShaderSource {
    frag: Option<String>,
    vert: Option<String>,
    geometry: Option<String>,
}

impl ShaderSource {
    fn new() -> Self {
        Self {
            frag: None,
            vert: None,
            geometry: None,
        }
    }

    fn get_shader_source(&self, shader_type: ShaderType) -> Option<String> {
        match shader_type {
            ShaderType::Vertex => self.vert.clone(),
            ShaderType::Fragment => self.frag.clone(),
            ShaderType::Geometry => self.geometry.clone(),
            _ => panic!("Unknown shader type"),
        }
    }
}

struct Shader {
    id: u32,
    program: u32,
    locations: HashMap<String, u32>,
}

impl Shader {
    pub fn new(filepath: &str) -> Self {
        todo!();
    }

    pub fn bind(&self) {
        todo!();
    }

    pub fn unbind(&self) {
        todo!();
    }

    pub fn set_uniform1f(&self, name: &str, value: f32) {
        todo!();
    }

    pub fn set_uniform1i(&self, name: &str, value: i32) {
        todo!();
    }

    pub fn set_uniform4f(&self, name: &str, value: glm::Vec4) {
        todo!();
    }

    fn get_uniform_location(&self, name: &str) -> u32 {
        todo!();
    }

    fn create_shader(&self, shader_source: &ShaderSource) -> u32 {
        let program: u32 = glCreateProgram();
        unsafe {
            let fragment_source = shader_source
                .get_shader_source(ShaderType::Fragment)
                .unwrap_or("".to_string());
            let vertex_source = shader_source
                .get_shader_source(ShaderType::Vertex)
                .unwrap_or("".to_string());
            let geometry_source = shader_source
                .get_shader_source(ShaderType::Geometry)
                .unwrap_or("".to_string());

            println!("Parsed fragment:\n {}\n", fragment_source);
            println!("Parsed vertex:\n {}\n", vertex_source);
            println!("Parsed geometry:\n {}\n", geometry_source);

            let fragment_shader = self.compile_shader(ShaderType::Fragment, &fragment_source);
            let vertex_shader = self.compile_shader(ShaderType::Vertex, &vertex_source);
            let geometry_shader = self.compile_shader(ShaderType::Geometry, &geometry_source);

            glAttachShader(program, fragment_shader);
            glAttachShader(program, vertex_shader);
            glAttachShader(program, geometry_shader);
            glLinkProgram(program);
            glValidateProgram(program);

            glDeleteShader(fragment_shader);
            glDeleteShader(vertex_shader);
            glDeleteShader(geometry_shader);
        }
        program
    }

    fn compile_shader(&self, shader_type: ShaderType, source: &str) -> u32 {
        todo!();
    }

    fn parse_shader(filepath: &str) -> ShaderSource {
        //TODO prevent copy
        let contents = std::fs::read_to_string(filepath).unwrap();
        let mut ret = ShaderSource::new();
        let mut current_shader = ShaderType::None;
        for line in contents.lines() {
            if line.starts_with("#shader") {
                if line == "#shader vertex" {
                    current_shader = ShaderType::Vertex;
                    ret.vert = Some("".to_string());
                } else if line == "#shader fragment" {
                    current_shader = ShaderType::Fragment;
                } else if line == "#shader geometry" {
                    current_shader = ShaderType::Geometry;
                }
            } else {
                match current_shader {
                    ShaderType::None => {
                        continue;
                    }
                    ShaderType::Vertex => {
                        let mut new_vert = ret.vert.clone().unwrap();
                        new_vert.push_str(line);
                        let _ = ret.vert.insert(new_vert);
                    }
                    ShaderType::Fragment => {
                        let mut new_frag = ret.frag.clone().unwrap();
                        new_frag.push_str(line);
                        let _ = ret.frag.insert(new_frag);
                    }
                    ShaderType::Geometry => {
                        let mut new_geom = ret.geometry.clone().unwrap();
                        new_geom.push_str(line);
                        let _ = ret.geometry.insert(new_geom);
                    }
                }
            }
        }
        ret
    }
}
