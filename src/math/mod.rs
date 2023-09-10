use crate::utils::default_construct::DefaultConstruct;

#[derive(Clone, Copy)]
pub struct Vec3<T: DefaultConstruct> {
    x: T,
    y: T,
    z: T,
}

impl<T: DefaultConstruct> Vec3<T> {
    pub fn new() -> Self {
        Self {
            x: T::new(),
            y: T::new(),
            z: T::new(),
        }
    }
}

pub type Vec3f = Vec3<f32>;
