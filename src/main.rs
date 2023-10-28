extern crate glm;
extern crate num;
extern crate sdl2;

mod ecs;
mod engine;
mod input;
mod math;
mod utils;

use gl::SCISSOR_TEST;
use gl::types::{GLfloat, GLint, GLuint};
use crate::ecs::*;
use crate::engine::logic::*;
use crate::engine::time::Time;
use crate::engine::*;
use crate::input::Input;
use crate::math::*;
use crate::num::*;
use glm::{clamp, cos, DVec3, GenNum, sin, Vec3};
use sdl2::keyboard::Scancode;

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
}

fn update_ecs_gravity_sys(
    state: &mut GravitySystemState,
    mut query: Query<(&mut Position, &mut Velocity, &Mass)>,
    ei: &EngineInterface,
    commands: &mut Commands,
) {
    for attractable in query.iter() {
        let mut sum_force = DVec3::zero();
        for attractor in query.iter() {
            if attractor.ent == attractable.ent {
                continue;
            }

            let to_attractor = attractor.comp.0 .0 - attractable.comp.0 .0;
            let distance = to_attractor.length();
            // TODO: shit! glm huita!
            let tmp = state.gravity_constant * attractor.comp.2 .0 * attractable.comp.2 .0
                / (distance * distance * distance);
            let force = to_attractor.map(|v| v * tmp);
            // TODO: glm is shit!
            sum_force = sum_force.zip(force, |v1, v2| v1 + v2);
        }
        // TODO!
    }
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

            if input.is_key_pressed(Scancode::Escape) {
                commands.queue_command(Command::Exit);
            }
            if time.get_time() - *last_fps_print_time > 1.0 {
                *last_fps_print_time = time.get_time();
                println!("FPS: {}", time.get_fps());
            }
            else {
                *last_fps_print_time -= 0.001;
            }
        }
        fn render(last_fps_print_time: &mut f64, ei: &EngineInterface, commands: &mut Commands) {
            unsafe {
                let f1 = sin(*last_fps_print_time as GLfloat * 352.0) / 2.0 + 0.5;
                let f2 = sin(*last_fps_print_time as GLfloat * 352.0) / 2.0 + 0.5;
                let f3 = sin(*last_fps_print_time as GLfloat * 123.0) / 2.0 + 0.5;
                let f4 = sin(*last_fps_print_time as GLfloat * 515.0) / 2.0 + 0.5;
                let f5 = sin(*last_fps_print_time as GLfloat * 612.0) / 2.0 + 0.5;
                let f6 = sin(*last_fps_print_time as GLfloat * 122.0) / 2.0 + 0.5;
                let f7 = sin(*last_fps_print_time as GLfloat * 612.0) / 2.0 + 0.5;
                let f8 = sin(*last_fps_print_time as GLfloat * 125.0) / 2.0 + 0.5;
                let i1 = (f1 * 120.0 + f2) as GLint;
                let i2 = (f2 * 200.0 + f1) as GLint;
                let i3 = (f3 * 225.0 + f3) as GLint;
                let i4 = (f4 * 155.0 + f4) as GLint;
                let i5 = (f5 * 112.0 + f6) as GLint;
                let i6 = (f6 * 225.0 + f5) as GLint;
                let i7 = (f7 * 61.0 + f7) as GLint;
                let i8 = (f8 * 51.0 + f1) as GLint;
                gl::Enablei(SCISSOR_TEST, 0);
                gl::Scissor(200+i1, 200+i2, i3, i4);
                gl::Viewport(200+i2, 200+i3, i4, i1);
                gl::ClearColor(sin(*last_fps_print_time as GLfloat * 15000.0) / 2.0 + 0.5,
                               sin(*last_fps_print_time as GLfloat * 15001.0) / 2.0 + 0.5,
                               sin(*last_fps_print_time as GLfloat * 15002.0) / 2.0 + 0.5, 1.0);
            }
        }
        basic_logic.add_function(update, LogicFuncType::Update);
        basic_logic.add_function(render, LogicFuncType::Render);
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
