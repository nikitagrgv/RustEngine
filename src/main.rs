extern crate glm;
extern crate sdl2;

mod ecs;
mod engine;
mod input;
mod utils;

use crate::ecs::*;
use crate::engine::*;

use crate::input::Input;
use glm::Vec3;
use sdl2::keyboard::Scancode;

#[derive(Clone, Copy, Debug)]
struct Position(Vec3);

#[derive(Clone, Copy, Debug)]
struct Mass(f32);

#[derive(Clone, Copy, Debug)]
struct Velocity(Vec3);

fn init_example(state: &mut i32, ei: &mut EngineInterface) {
    println!("inited!");
}

fn update_example(state: &mut i32, ei: &mut EngineInterface) {
    let input = ei.get_subsystem::<Input>();
    if input.is_key_pressed(Scancode::W) {
        println!("PRESSED!");
    }
    if input.is_key_released(Scancode::W) {
        println!("RELEASED!");
    }
    if input.is_key_down(Scancode::A) {
        println!("DOWN!");
    }
}

fn post_update_example(state: &mut i32, ei: &mut EngineInterface) {}

fn shutdown_example(state: &mut i32, ei: &mut EngineInterface) {
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
