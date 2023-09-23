mod ecs;
mod math;
mod utils;

use crate::ecs::*;
use crate::math::*;
use crate::utils::combo_iterator::*;
use std::collections::HashMap;
use std::iter::Zip;


#[derive(Clone, Copy, Default)]
struct Position(Vec3f);

#[derive(Clone, Copy, Default)]
struct Mass(f32);

#[derive(Clone, Copy, Default)]
struct Velocity(Vec3f);

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
    println!("aa");
}
