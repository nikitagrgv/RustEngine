use std::os::raw::c_void;

use gl33::gl_core_types;
use gl33::global_loader::{glBindBuffer, glBufferData, glGenBuffers};

pub struct VertexBuffer {
    id: u32,
}

impl VertexBuffer {
    fn new(data: *const c_void, size: i32) -> Self {
        let mut id = 0;
        unsafe {
            glGenBuffers(size, &mut id);
            glBindBuffer(gl33::gl_enumerations::GL_ARRAY_BUFFER, id);
            glBufferData(
                gl33::gl_enumerations::GL_ARRAY_BUFFER,
                size as isize,
                data,
                gl33::gl_enumerations::GL_STATIC_DRAW,
            );
        };
        Self { id }
    }

    fn bind(self) {
        todo!();
    }

    fn unbind(self) {
        todo!();
    }
}
