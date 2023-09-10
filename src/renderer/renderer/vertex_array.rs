use gl33::global_loader::{glGenVertexArrays, glDeleteVertexArrays, glBindVertexArray};

use super::vertex_buffer::VertexBuffer;

struct VertexArray {
    id : u32,
}

impl VertexArray {
    fn new() -> Self {
        let mut id = 0;
        unsafe {
            glGenVertexArrays(1, &mut id);
        }
        Self{id}
    }

    fn bind(&self) {
        unsafe {
            glBindVertexArray(self.id);           
        }
    }
    
    fn unbind()
    {
        unsafe {
            glBindVertexArray(0);
        }
    }

    fn bind_buffer(&self, buffer : VertexBuffer)
    {
        unsafe {
            self.bind();
            buffer.bind();
        }

        todo!();
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            glDeleteVertexArrays(1, &self.id);
        }
    }
}
