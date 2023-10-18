mod ecs;
mod engine;
mod math;
mod utils;

use crate::ecs::*;
use crate::engine::*;
use crate::math::*;

#[derive(Clone, Copy, Default, Debug)]
struct Position(Vec3f);

impl Position {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(Vec3f::new(x, y, z))
    }
}

#[derive(Clone, Copy, Default, Debug)]
struct Mass(f32);

impl Mass {
    pub fn new(mass: f32) -> Self {
        Self(mass)
    }
}

#[derive(Clone, Copy, Default, Debug)]
struct Velocity(Vec3f);

impl Velocity {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(Vec3f::new(x, y, z))
    }
}


use bevy::prelude::{*};

// fn greet_people(query: Query<&Name, With<Person>>) {
//     for name in &query {
//         println!("hello {}!", name.0);
//     }
// }



fn main() {
    // bevy::app::App::new().add_systems()


    let mut engine = Engine::new();
    engine.run();


}
