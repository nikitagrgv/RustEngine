extern crate glm;
extern crate sdl2;

mod ecs;
mod engine;
mod input;
mod utils;

use crate::ecs::*;
use crate::engine::logic::*;
use crate::engine::*;

use crate::engine::time::Time;
use crate::input::Input;
use glm::{clamp, Vec3};
use sdl2::keyboard::Scancode;
use sdl2::libc::stat;

#[derive(Clone, Copy, Debug)]
struct Position(Vec3);

#[derive(Clone, Copy, Debug)]
struct Mass(f32);

#[derive(Clone, Copy, Debug)]
struct Velocity(Vec3);

struct ExampleState {
    red: f32,
    green: f32,
    last_time: f64,
}

fn init_example(state: &mut ExampleState, ei: &mut EngineInterface) {
    println!("inited!");
}

fn update_example(state: &mut ExampleState, ei: &mut EngineInterface) {
    let input = ei.get_subsystem::<Input>();
    let time = ei.get_subsystem::<Time>();

    if input.is_key_down(Scancode::Left) {
        state.red -= 0.01f32;
    }

    if input.is_key_down(Scancode::Right) {
        state.red += 0.01f32;
    }

    state.red = clamp(state.red, 0f32, 1f32);

    let cur_time = time.get_time();
    println!("fps: {}", time.get_fps());
    if cur_time - state.last_time > 0.1f64 {
        state.last_time = cur_time;
        if state.green > 0.5 {
            state.green = 0f32;
        } else {
            state.green = 1f32;
        }
    }

    unsafe { gl::ClearColor(state.red, state.green, 0f32, 1.0) };
}

fn post_update_example(state: &mut ExampleState, ei: &mut EngineInterface) {}

fn render_example(state: &mut ExampleState, ei: &mut EngineInterface) {}

fn shutdown_example(state: &mut ExampleState, ei: &mut EngineInterface) {
    println!("shutdown!");
}

fn main() {
    let mut engine = Engine::new();

    let mut logic = StateLogic::new(ExampleState {
        red: 0f32,
        green: 0f32,
        last_time: 0f64,
    });
    logic.add_function(init_example, LogicFuncType::Init);
    logic.add_function(update_example, LogicFuncType::Update);
    logic.add_function(post_update_example, LogicFuncType::PostUpdate);
    logic.add_function(render_example, LogicFuncType::Render);
    logic.add_function(shutdown_example, LogicFuncType::Shutdown);
    engine.add_logic(logic);

    engine.run();
}
