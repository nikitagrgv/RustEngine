extern crate glm;
extern crate sdl2;

mod ecs;
mod engine;
mod input;
mod utils;

use crate::ecs::*;
use crate::engine::logic::*;
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

struct ExampleState {
    var: f32,
}

fn init_example(state: &mut ExampleState, ei: &mut EngineInterface) {
    println!("inited!");
}

fn update_example(state: &mut ExampleState, ei: &mut EngineInterface) {
    let input = ei.get_subsystem::<Input>();

    if input.is_key_down(Scancode::Left) {
        state.var -= 0.01f32;
    }

    if input.is_key_down(Scancode::Right) {
        state.var += 0.01f32;
    }

    if state.var > 1f32 || state.var < 0f32
    {
        state.var = 0f32;
    }

    unsafe { gl::ClearColor(state.var, 0f32, 0f32, 1.0) };
}

fn post_update_example(state: &mut ExampleState, ei: &mut EngineInterface) {

}

fn render_example(state: &mut ExampleState, ei: &mut EngineInterface) {

}

fn shutdown_example(state: &mut ExampleState, ei: &mut EngineInterface) {
    println!("shutdown!");
}

fn main() {
    let mut engine = Engine::new();

    let mut logic = StateLogic::new(ExampleState { var: 0f32 });
    logic.add_function(init_example, LogicFuncType::Init);
    logic.add_function(update_example, LogicFuncType::Update);
    logic.add_function(post_update_example, LogicFuncType::PostUpdate);
    logic.add_function(render_example, LogicFuncType::Render);
    logic.add_function(shutdown_example, LogicFuncType::Shutdown);
    engine.add_logic(logic);

    engine.run();
}
