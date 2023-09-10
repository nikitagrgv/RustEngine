struct VertexBufferLayoutElement {
    element_type : gl33::GLenum,
    count : u32,
    normalized : bool,
}

impl VertexBufferLayoutElement {
    fn new(element_type: gl33::GLenum, count: u32, normalized: bool) -> Self {
        Self {
            element_type,
            count,
            normalized,
        }
    }

    fn get_type_size(element_type : gl33::GLenum) -> Result<u32, String> {
        match element_type {
            gl33::gl_enumerations::GL_FLOAT => Ok(4),
            gl33::gl_enumerations::GL_UNSIGNED_INT => Ok(4),
            gl33::gl_enumerations::GL_UNSIGNED_BYTE => Ok(1),
            _ => Err("Unknown vertex buffer layout element type".to_string()),
        }
    }
}

pub struct VertexBufferLayout {
    elements : Vec<VertexBufferLayoutElement>,
    stride : u32,
}

impl VertexBufferLayout {
    fn new() -> Self {
        Self {
            stride : 0, 
            elements : Vec::new(),
        }
    }

    fn push(&mut self, count : u32, element_type: gl33::GLenum) {
        self.stride += count * VertexBufferLayoutElement::get_type_size(element_type).unwrap();
        self.elements.push(VertexBufferLayoutElement::new(element_type, count, false));
    }
}
