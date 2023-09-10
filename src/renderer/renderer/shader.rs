use std::collections::hash_map::HashMap;

enum ShaderType {
    None,
    Vertex(String),
    Fragment(String),
    Geometry(String),
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

    pub fn setUniform1f(&self, name: &str, value: f32) {
        todo!();
    }

    pub fn setUniform1i(&self, name: &str, value: i32) {
        todo!();
    }

    pub fn setUniform4f(&self, name: &str, value: glm::Vec4) {
        todo!();
    }

    fn get_uniform_location(&self, name: &str) -> u32 {
        todo!();
    }

    fn parse(filepath: &str) -> ShaderType {
        todo!();
    }
}

