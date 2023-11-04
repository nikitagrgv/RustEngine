extern crate glm;
extern crate num;
extern crate sdl2;

mod engine;
mod input;
mod math;
mod utils;
mod world;

use crate::engine::logic::*;
use crate::engine::time::Time;
use crate::engine::*;
use crate::input::Input;
use crate::math::*;
use crate::num::*;
use crate::world::*;
use gl::types::{GLfloat, GLint, GLuint};
use gl::SCISSOR_TEST;
use glm::{clamp, cos, sin, DVec2, DVec3, GenNum, IVec2, UVec2, Vec2, Vec3};
use sdl2::keyboard::Scancode;
use std::ops::{Deref, DerefMut};

macro_rules! thing_component_wrapper {
    ($name: ident, $base: ty) => {
        #[derive(Clone, Copy, Debug)]
        struct $name($base);

        impl Deref for $name {
            type Target = $base;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

thing_component_wrapper!(Position, DVec3);
thing_component_wrapper!(Mass, f64);
thing_component_wrapper!(Velocity, DVec3);

struct GravitySystemState {
    gravity_constant: f64,

    center: DVec2,
    scale: f64,

    cur_trail_num: usize,
    trails: Vec<DVec3>,
}

impl GravitySystemState {
    // TODO: no window_size
    fn to_screen_coords(&self, coords: DVec2, window_size: UVec2) -> IVec2 {
        let rel_coords = coords - self.center;
        IVec2::new(
            (rel_coords.x * self.scale) as i32 + ((window_size.x / 2) as i32),
            (rel_coords.y * self.scale) as i32 + ((window_size.y / 2) as i32),
        )
    }
}

fn init_gravity_sys(state: &mut GravitySystemState, ei: &EngineInterface, commands: &mut Commands) {
    println!("inited!");
}

fn update_gravity_sys(
    state: &mut GravitySystemState,
    ei: &EngineInterface,
    commands: &mut Commands,
) {
    let input = ei.get_subsystem::<Input>();
    let time = ei.get_subsystem::<Time>();
    let dt = time.get_delta();

    if input.is_key_down(Scancode::Q) {
        state.scale /= (1.0 + 4.0 * dt);
    }
    if input.is_key_down(Scancode::E) {
        state.scale *= (1.0 + 4.0 * dt);
    }

    let mut move_dir = DVec2::zero();
    if input.is_key_down(Scancode::A) {
        move_dir.x -= 1.0;
    }
    if input.is_key_down(Scancode::D) {
        move_dir.x += 1.0;
    }
    if input.is_key_down(Scancode::W) {
        move_dir.y += 1.0;
    }
    if input.is_key_down(Scancode::S) {
        move_dir.y -= 1.0;
    }
    move_dir = move_dir / state.scale * 10.0;
    state.center = state.center + move_dir;
}

fn update_ecs_gravity_sys(
    state: &mut GravitySystemState,
    mut query: Query<(&mut Position, &mut Velocity, &Mass)>,
    ei: &EngineInterface,
    commands: &mut Commands,
) {
    let time = ei.get_subsystem::<Time>();
    let dt = time.get_delta();

    let mut attractable_iterator = query.iter_mut();
    while let Some(attractable) = attractable_iterator.next() {
        let mut sum_accel = DVec3::zero();
        for attractor in attractable_iterator.iter_skipping_current() {
            let to_attractor = attractor.comp.0 .0 - attractable.comp.0 .0;
            let distance = to_attractor.length();
            let force = to_attractor
                * (state.gravity_constant * attractor.comp.2 .0 / (distance * distance * distance));
            sum_accel = sum_accel + force;
        }
        attractable.comp.1 .0 = attractable.comp.1 .0 + sum_accel * dt;
    }

    for obj in query.iter_mut() {
        obj.comp.0 .0 = obj.comp.0 .0 + obj.comp.1 .0 * dt;
    }

    // if (state.trails.len() > 10000) {
    //     for obj in query.iter() {
    //         state.trails(obj.comp.0 .0);
    //     }
    // } else {
    //     for obj in query.iter() {
    //         state.trails.push(obj.comp.0 .0);
    //     }
    // }
    for obj in query.iter() {
        if state.cur_trail_num >= 10000 {
            state.cur_trail_num = 0;
        }
        let pos = obj.comp.0 .0;
        if state.cur_trail_num >= state.trails.len() {
            state.trails.push(pos);
        } else {
            state.trails[state.cur_trail_num] = pos;
        }
        state.cur_trail_num += 1;
    }
    println!("TRAILS : {}", state.trails.len())
}

fn print_positions(
    state: &mut GravitySystemState,
    mut query: Query<(&Position, &Velocity)>,
    ei: &EngineInterface,
    commands: &mut Commands,
) {
    // for obj in query.iter() {
    //     println!("ent: {} | pos = {:?}", obj.ent.to_num(), obj.comp.0 .0);
    // }
}

fn render_positions(
    state: &mut GravitySystemState,
    mut query: Query<(&Position)>,
    ei: &EngineInterface,
    commands: &mut Commands,
) {
    const OBJ_SIZE: i32 = 10;
    const HALF_SIZE: i32 = OBJ_SIZE / 2;

    let window = ei.get_subsystem::<Window>();
    let ws = window.get_size();

    unsafe {
        gl::Disablei(SCISSOR_TEST, 0);
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        // render trails
        for trail in &state.trails {
            let pos = state.to_screen_coords(DVec2::new(trail.x, trail.y), ws);
            gl::Enablei(SCISSOR_TEST, 0);
            gl::Scissor(
                pos.x - HALF_SIZE / 2,
                pos.y - HALF_SIZE / 2,
                OBJ_SIZE / 2,
                OBJ_SIZE / 2,
            );
            gl::ClearColor(0.6, 0.3, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        for obj in query.iter() {
            let pos = state.to_screen_coords(DVec2::new(obj.comp.0.x, obj.comp.0.y), ws);
            gl::Enablei(SCISSOR_TEST, 0);
            gl::Scissor(pos.x - HALF_SIZE, pos.y - HALF_SIZE, OBJ_SIZE, OBJ_SIZE);
            gl::ClearColor(0.9, 0.9, 0.9, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
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
            1e16,
            DVec3::new(0.0, 0.0, 0.0),
        );
        create_phys_entity(
            world,
            DVec3::new(0.0, 20.0, 0.0),
            1e9,
            DVec3::new(-240.0, 0.0, 0.0),
        );
        create_phys_entity(
            world,
            DVec3::new(200.0, 0.0, 0.0),
            1e9,
            DVec3::new(0.0, 30.0, 0.0),
        );
        create_phys_entity(
            world,
            DVec3::new(0.0, 100.0, 0.0),
            1e9,
            DVec3::new(50.0, 0.0, 0.0),
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
            } else {
                *last_fps_print_time -= 0.001;
            }
        }
        basic_logic.add_function(update, LogicFuncType::Update);
        engine.add_logic(basic_logic);
    }

    {
        let gravity_state = GravitySystemState {
            gravity_constant: 6.6743e-11,
            center: DVec2::zero(),
            scale: 1.0,
            cur_trail_num: 0,
            trails: Vec::new(),
        };
        let mut gravity_logic = StateLogic::new(gravity_state);
        gravity_logic.add_function(init_gravity_sys, LogicFuncType::Init);
        gravity_logic.add_ecs_function(print_positions, LogicFuncType::Init);
        gravity_logic.add_function(update_gravity_sys, LogicFuncType::Update);
        gravity_logic.add_ecs_function(update_ecs_gravity_sys, LogicFuncType::Update);
        gravity_logic.add_ecs_function(print_positions, LogicFuncType::PostUpdate);
        gravity_logic.add_ecs_function(render_positions, LogicFuncType::Render);
        engine.add_logic(gravity_logic);
    }

    engine.run();
}
