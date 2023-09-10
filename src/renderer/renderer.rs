use crate::renderer::renderer::vertex_buffer::VertexBuffer;

pub mod vertex_buffer;
pub mod shader;
pub mod index_buffer;
pub mod vertex_buffer_layout;
pub mod vertex_array;

struct Renderer {
}

impl Renderer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw(vb : VertexBuffer)
    {
    }
}
