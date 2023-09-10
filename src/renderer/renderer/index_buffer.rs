struct IndexBuffer {
    id: u32,
    count: u32,
}

impl IndexBuffer {
    fn new(id: u32, count: u32) -> Self {
        Self { id, count }
    }
    fn bind(self) {
        todo!();
    }
    fn unbind(self) {
        todo!();
    }
}
