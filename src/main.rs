extern crate glm;
extern crate sdl2;

mod ecs;
mod engine;
mod input;
mod utils;

use crate::ecs::*;
use crate::engine::*;

use glm::Vec3;

#[derive(Clone, Copy, Debug)]
struct Position(Vec3);

#[derive(Clone, Copy, Debug)]
struct Mass(f32);

#[derive(Clone, Copy, Debug)]
struct Velocity(Vec3);

fn init_example(state: &mut i32, engine_interface: &mut EngineInterface) {
    println!("inited!");
}

fn update_example(state: &mut i32, engine_interface: &mut EngineInterface) {
    
}

fn post_update_example(state: &mut i32, engine_interface: &mut EngineInterface) {
}

fn shutdown_example(state: &mut i32, engine_interface: &mut EngineInterface) {
    println!("shutdown!");
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
