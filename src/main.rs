mod ecs;
mod math;
mod utils;

use crate::ecs::*;
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

// use bevy::prelude::{*};
// fn greet_people(query: Query<&Name, With<Person>>) {
//     for name in &query {
//         println!("hello {}!", name.0);
//     }
// }

// trait Query
// {
//     type Item;
//
//     fn fetch() -> Self::Item;
// }

fn main() {
    let mut world = World::new();
    world.register_component::<Position>();
    world.register_component::<Mass>();

    let e0 = world.create_entity();
    let e1 = world.create_entity();

    world.add_component(Position::new(1.0, 2.0, 3.0), e0);
    world.add_component(Mass::new(4.0), e0);

    world.add_component(Position::new(7.0, 8.0, 9.0), e1);
    world.add_component(Mass::new(10.0), e1);

    let mut q = world.query_mut::<(&Position, &Mass)>();
    if let Some(comps) = q.fetch_entity(e0) {
        println!("{:#?}", comps);
    }
    if let Some(comps) = q.fetch_entity(e1) {
        println!("{:#?}", comps);
    }


}
