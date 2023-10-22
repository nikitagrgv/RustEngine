extern crate glm;

use glm::{DVec2, DVec3, Vec2, Vec3};

/// Length
pub trait Length {
    type Ret;
    fn length2(&self) -> Self::Ret;
    fn length(&self) -> Self::Ret;
}

impl Length for Vec2 {
    type Ret = f32;

    fn length2(&self) -> Self::Ret {
        self.x * self.x + self.y * self.y
    }

    fn length(&self) -> Self::Ret {
        glm::sqrt(self.length2())
    }
}

impl Length for Vec3 {
    type Ret = f32;

    fn length2(&self) -> Self::Ret {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn length(&self) -> Self::Ret {
        glm::sqrt(self.length2())
    }
}

impl Length for DVec2 {
    type Ret = f64;

    fn length2(&self) -> Self::Ret {
        self.x * self.x + self.y * self.y
    }

    fn length(&self) -> Self::Ret {
        glm::sqrt(self.length2())
    }
}

impl Length for DVec3 {
    type Ret = f64;

    fn length2(&self) -> Self::Ret {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn length(&self) -> Self::Ret {
        glm::sqrt(self.length2())
    }
}
