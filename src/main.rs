mod ecs;
mod math;
mod utils;

use crate::ecs::*;
use crate::math::*;

#[derive(Clone, Copy, Default)]
struct Position(Vec3f);

#[derive(Clone, Copy, Default)]
struct Mass(f32);

#[derive(Clone, Copy, Default)]
struct Velocity(Vec3f);


fn main() {
}
