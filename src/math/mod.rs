#[derive(Clone, Copy, Default)]
pub struct Vec3<T: Default> {
    x: T,
    y: T,
    z: T,
}

pub type Vec3f = Vec3<f32>;
