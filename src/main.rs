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

// use bevy::prelude::{*};
//
// // fn greet_people(query: Query<&Name, With<Person>>) {
// //     for name in &query {
// //         println!("hello {}!", name.0);
// //     }
// // }

fn init_example(state: &mut i32, commands: &mut Commands) {
    println!("init {}", state);
}

fn update_example(state: &mut i32, commands: &mut Commands) {
    *state += 1;
    if *state >= 150
    {
        println!("stop! {}", state);
        commands.add(Command::Exit);
    }

    println!("update {}", state);
}

fn post_update_example(state: &mut i32, commands: &mut Commands) {
    println!("post update {}", state);
}

fn shutdown_example(state: &mut i32, commands: &mut Commands) {
    println!("shutdown {}", state);
}

fn main() {
    let mut engine = Engine::new();

    let mut logic = StateLogic::<i32>::new(123);
    logic.add_function(init_example, LogicFuncType::Init);
    logic.add_function(update_example, LogicFuncType::Update);
    logic.add_function(post_update_example, LogicFuncType::PostUpdate);
    logic.add_function(shutdown_example, LogicFuncType::Shutdown);
    engine.add_logic(logic);

    engine.run();
}
