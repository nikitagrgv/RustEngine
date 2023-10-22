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
use crate::utils::scoped_perf::ScopedPerf;
use glm::{clamp, DVec3, Vec3};
use sdl2::keyboard::Scancode;
use sdl2::keyboard::Scancode::Escape;

#[derive(Clone, Copy, Debug)]
struct Position(DVec3);

#[derive(Clone, Copy, Debug)]
struct Mass(f64);

#[derive(Clone, Copy, Debug)]
struct Velocity(DVec3);

struct GravitySystemState {
    gravity_constant: f64,
}

fn init_gravity_sys(state: &mut GravitySystemState, ei: &EngineInterface, commands: &mut Commands) {
    println!("inited!");
}

fn update_gravity_sys(
    state: &mut GravitySystemState,
    ei: &EngineInterface,
    commands: &mut Commands,
) {
    // let input = ei.get_subsystem::<Input>();
    // let time = ei.get_subsystem::<Time>();
    //
    // if input.is_key_down(Scancode::Left) {
    //     state.red -= 0.01f32;
    // }
    //
    // if input.is_key_down(Scancode::Right) {
    //     state.red += 0.01f32;
    // }
    //
    // state.red = clamp(state.red, 0f32, 1f32);
    //
    // let cur_time = time.get_time();
    // println!("fps: {}", time.get_fps());
    // if cur_time - state.last_time > 1f64 {
    //     state.last_time = cur_time;
    //     if state.green > 0.5 {
    //         state.green = 0f32;
    //     } else {
    //         state.green = 1f32;
    //     }
    // }
    //
    // state.num += 1;
    // if (state.num >= 10) {
    //     ei.queue_command(Command::Exit);
    // }
    //
    // unsafe { gl::ClearColor(state.red, state.green, state.blue, 1.0) };
}

fn update_ecs_gravity_sys(
    state: &mut GravitySystemState,
    mut query: Query<(&mut Position, &mut Velocity, &Mass)>,
    ei: &EngineInterface,
    commands: &mut Commands,
) {
}

fn main() {
    let mut engine = Engine::new();

    let mut world = engine.get_subsystem_mut::<World>();
    world.register_component::<Position>();
    world.register_component::<Mass>();
    world.register_component::<Velocity>();

    {
        fn create_phys_entity(world: &mut World, pos: DVec3, mass: f64, vel: DVec3) {
            let e = world.create_entity();
            world.set_component(Position(pos), e);
            world.set_component(Mass(mass), e);
            world.set_component(Velocity(vel), e);
        }

        create_phys_entity(
            world,
            DVec3::new(0.0, 0.0, 0.0),
            1e10,
            DVec3::new(0.0, 0.0, 0.0),
        );
        create_phys_entity(
            world,
            DVec3::new(1e6, 0.0, 0.0),
            1e5,
            DVec3::new(0.0, 1e6, 0.0),
        );
        create_phys_entity(
            world,
            DVec3::new(0.0, 1e6, 0.0),
            1e7,
            DVec3::new(1e6, 0.0, 0.0),
        );
    }

    {
        let mut basic_logic = StateLogic::new(0f64);
        fn update(last_fps_print_time: &mut f64, ei: &EngineInterface, commands: &mut Commands) {
            let time = ei.get_subsystem::<Time>();
            let input = ei.get_subsystem::<Input>();

            if input.is_key_pressed(Escape) {
                commands.queue_command(Command::Exit);
            }
            if time.get_time() - *last_fps_print_time > 1.0 {
                *last_fps_print_time = time.get_time();
                println!("FPS: {}", time.get_fps());
            }
        }
        basic_logic.add_function(update, LogicFuncType::Update);
        engine.add_logic(basic_logic);
    }

    {
        let gravity_state = GravitySystemState {
            gravity_constant: 6.6743e-11,
        };
        let mut gravity_logic = StateLogic::new(gravity_state);
        gravity_logic.add_function(init_gravity_sys, LogicFuncType::Init);
        gravity_logic.add_function(update_gravity_sys, LogicFuncType::Update);
        gravity_logic.add_ecs_function(update_ecs_gravity_sys, LogicFuncType::Update);
        engine.add_logic(gravity_logic);
    }

    engine.run();
}
