#[derive(Clone, Copy, Debug, Default)]
pub struct Vec3<T: Default> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Default> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

pub type Vec3f = Vec3<f32>;
