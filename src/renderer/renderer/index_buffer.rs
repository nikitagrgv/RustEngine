use std::os::raw::c_void;

use gl33::global_loader::{glGenBuffers, glBindBuffer, glBufferData, glDeleteBuffers};

struct IndexBuffer {
    id: u32,
    count: u32,
}

impl IndexBuffer {
    fn new(data: *const c_void, count: u32) -> Self {
        let mut id = 0;
        unsafe {
            glGenBuffers(1, &mut id);
            glBindBuffer(gl33::gl_enumerations::GL_ELEMENT_ARRAY_BUFFER, id);
            glBufferData(
                gl33::gl_enumerations::GL_ELEMENT_ARRAY_BUFFER,
                count as isize * std::mem::size_of::<u32>() as isize,
                data,
                gl33::gl_enumerations::GL_STATIC_DRAW,
            );
        }
        Self { id, count }
    }
    fn bind(self) {
        todo!();
    }
    fn unbind(self) {
        todo!();
    }
}

impl Drop for IndexBuffer {
    fn drop(&mut self) {
        unsafe {
            glDeleteBuffers(1, &self.id);
        }
    }
}
