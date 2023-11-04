extern crate nalgebra_glm as glm;
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
use glm::{clamp, cos, sin, DMat4, DVec2, DVec3, DVec4, IVec2, UVec2, Vec2, Vec3};
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

    cur_trail_num: usize,
    trails: Vec<DVec3>,

    camera_transform: glm::DMat4,
    proj_view: glm::DMat4,
}

impl GravitySystemState {
    // TODO: no window_size
    fn to_screen_coords(&self, coords: DVec3, window_size: UVec2) -> Option<IVec2> {
        let coords = DVec4::new(coords.x, coords.y, coords.z, 1.0);
        let screen_coords = self.proj_view * coords;

        let w = screen_coords.w;
        let pos_x = screen_coords.x / w;
        let pos_y = screen_coords.y / w;
        let pos_z = screen_coords.z / w;

        if !(-1.0..1.0).contains(&pos_x)
            || !(-1.0..1.0).contains(&pos_y)
            || !(-1.0..1.0).contains(&pos_z)
        {
            return None;
        }

        Some(IVec2::new(
            (pos_x * window_size.x as f64 / 2.0 + window_size.x as f64 / 2.0) as i32,
            (pos_y * window_size.y as f64 / 2.0 + window_size.y as f64 / 2.0) as i32,
        ))
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

    let mut dir = DVec4::zero();
    if input.is_key_down(Scancode::A) {
        dir.x -= 1.0;
    }
    if input.is_key_down(Scancode::D) {
        dir.x += 1.0;
    }
    if input.is_key_down(Scancode::W) {
        dir.z -= 1.0;
    }
    if input.is_key_down(Scancode::S) {
        dir.z += 1.0;
    }
    if input.is_key_down(Scancode::Q) {
        dir.y -= 1.0;
    }
    if input.is_key_down(Scancode::E) {
        dir.y += 1.0;
    }

    let multiplier = if input.is_key_down(Scancode::LShift) {
        50.0
    } else {
        8.0
    };

    state.camera_transform = state
        .camera_transform
        .prepend_translation(&(dir.xyz() * dt * multiplier));

    {
        let mut delta_pitch = 0.0;
        if input.is_key_down(Scancode::Left) {
            delta_pitch += 1.0;
        }
        if input.is_key_down(Scancode::Right) {
            delta_pitch -= 1.0;
        }

        let mut delta_yaw = 0.0;
        if input.is_key_down(Scancode::Up) {
            delta_yaw -= 1.0;
        }
        if input.is_key_down(Scancode::Down) {
            delta_yaw += 1.0;
        }
        let multiplier = 15.0.to_radians();

        let pos = state.camera_transform.column(3);
        let pitch_rot = glm::rotation(delta_pitch * dt * multiplier, &DVec3::y_axis());
        let yaw_rot = glm::rotation(delta_yaw * dt * multiplier, &DVec3::x_axis());
        let mut new_transform = pitch_rot * state.camera_transform * yaw_rot;
        new_transform.set_column(3, &pos);
        state.camera_transform = new_transform;

        // println!("pos: {}", state.camera_transform.column(3));
    }
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
            let distance = to_attractor.magnitude();
            let force = to_attractor
                * (state.gravity_constant * attractor.comp.2 .0 / (distance * distance * distance));
            sum_accel = sum_accel + force;
        }
        attractable.comp.1 .0 = attractable.comp.1 .0 + sum_accel * dt;
    }

    for obj in query.iter_mut() {
        obj.comp.0 .0 = obj.comp.0 .0 + obj.comp.1 .0 * dt;
    }

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
    // println!("TRAILS : {}", state.trails.len())
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
    let aspect = ws.x as f64 / ws.y as f64;
    let proj = glm::perspective(aspect, 60.0.to_radians(), 0.01, 1000000.0);
    let view = state.camera_transform.try_inverse().unwrap();
    state.proj_view = proj * view;

    unsafe {
        gl::Disablei(SCISSOR_TEST, 0);
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        // render trails
        for trail in &state.trails {
            let pos = match state.to_screen_coords(*trail, ws) {
                None => continue,
                Some(p) => p,
            };
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
            let pos = match state.to_screen_coords(obj.comp.0, ws) {
                None => continue,
                Some(p) => p,
            };
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

        create_phys_entity(
            world,
            DVec3::new(0.0, 0.0, 0.0) + DVec3::new(160.0, 160.0, 0.0),
            5e13,
            DVec3::new(0.0, 0.0, 0.0) + DVec3::new(40.0, -40.0, 0.0),
        );
        create_phys_entity(
            world,
            DVec3::new(0.0, 20.0 / 10.0, 0.0) + DVec3::new(160.0, 160.0, 0.0),
            1e7,
            DVec3::new(-47.0, 0.0, 0.0) + DVec3::new(40.0, -40.0, 0.0),
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
            cur_trail_num: 0,
            trails: Vec::new(),
            camera_transform: DMat4::one(),
            proj_view: DMat4::one(),
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
