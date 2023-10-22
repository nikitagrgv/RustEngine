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
    num: i32,
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
    if cur_time - state.last_time > 1f64 {
        state.last_time = cur_time;
        if state.green > 0.5 {
            state.green = 0f32;
        } else {
            state.green = 1f32;
        }
    }

    state.num += 1;
    if (state.num >= 60) {
        ei.queue_command(Command::Exit);
    }

    unsafe { gl::ClearColor(state.red, state.green, 0f32, 1.0) };
}

fn update_ecs_example(
    state: &mut ExampleState,
    mut query: Query<&mut Position>,
    ei: &mut EngineInterface,
) {
    for c in query.iter_mut() {
        c.comp.0.x += 1.0;
    }

    // for c in query.iter() {
    //     println!("e {}: {}", c.ent.to_num(), c.comp.0.x);
    // }
}

extern crate pprof;

fn main() {
    {
        let guard = pprof::ProfilerGuard::new(10000).unwrap();

        let mut engine = Engine::new();
        let mut world = engine.get_subsystem_mut::<World>();
        world.register_component::<Position>();
        for i in 0..1000000 {
            let e = world.create_entity();
            world.set_component(Position(Vec3::new(0f32, 0f32, 0f32)), e);
        }

        let mut logic = StateLogic::new(ExampleState {
            red: 0f32,
            green: 0f32,
            last_time: 0f64,
            num: 0,
        });
        logic.add_function(init_example, LogicFuncType::Init);
        logic.add_function(update_example, LogicFuncType::Update);
        logic.add_ecs_function(update_ecs_example, LogicFuncType::Update);
        engine.add_logic(logic);

        engine.run();

        if let Ok(report) = guard.report().build() {
            use std::io::Write;
            let file = std::fs::File::create("/home/nikita/IdeaProjects/rustengine/data/flamegraph.svg").unwrap();
            let mut options = pprof::flamegraph::Options::default();
            options.image_width = Some(2500);
            report.flamegraph_with_options(file, &mut options).unwrap();
        };
    }
}
