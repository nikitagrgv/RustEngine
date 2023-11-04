use glm::DMat4;
use std::ops::{Deref, DerefMut};

pub struct Transform(pub DMat4);

impl Deref for Transform {
    type Target = DMat4;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Transform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
