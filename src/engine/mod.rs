mod engine_subsystem;
pub mod logic;
pub mod time;

extern crate gl;

use crate::engine::engine_subsystem::EngineSubsystem;
use crate::engine::logic::{Logic, LogicFuncType, StateLogic, StateObject};
use crate::engine::time::Time;
use crate::input::*;
use crate::utils;
use crate::world::{Entity, World};
use glm::{IVec2, UVec2};
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};

pub enum Command {
    Exit,
}

pub struct Commands {
    commands: Vec<Command>,
}

impl Commands {
    fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    pub fn queue_command(&mut self, command: Command) {
        self.commands.push(command);
    }
}

pub struct EngineInterface<'a> {
    engine: &'a Engine,
}

impl<'a> EngineInterface<'a> {
    pub fn new(engine: &'a Engine) -> Self {
        Self { engine }
    }

    pub fn get_subsystem<T: EngineSubsystem>(&self) -> &T {
        self.engine.get_subsystem()
    }
}

pub struct Window {
    sdl_context: sdl2::Sdl,
    sdl_video: sdl2::VideoSubsystem,
    sdl_window: sdl2::video::Window,
    gl_context: sdl2::video::GLContext,
}

impl Window {
    pub fn get_size(&self) -> UVec2 {
        let size = self.sdl_window.size();
        UVec2::new(size.0, size.1)
    }

    pub fn get_width(&self) -> u32 {
        self.sdl_window.size().0
    }

    pub fn get_height(&self) -> u32 {
        self.sdl_window.size().1
    }
}

pub struct Engine {
    world: World,
    exit_flag: bool,
    logics: Vec<Box<dyn Logic>>,
    window: Window,
    input: Input,
    time: Time,
}

impl EngineSubsystem for Input {
    fn get<'a>(engine: &'a Engine) -> &'a Self {
        &engine.input
    }

    fn get_mut<'a>(engine: &'a mut Engine) -> &'a mut Self {
        &mut engine.input
    }
}

impl EngineSubsystem for Time {
    fn get<'a>(engine: &'a Engine) -> &'a Self {
        &engine.time
    }

    fn get_mut<'a>(engine: &'a mut Engine) -> &'a mut Self {
        &mut engine.time
    }
}

impl EngineSubsystem for World {
    fn get<'a>(engine: &'a Engine) -> &'a Self {
        &engine.world
    }

    fn get_mut<'a>(engine: &'a mut Engine) -> &'a mut Self {
        &mut engine.world
    }
}

impl EngineSubsystem for Window {
    fn get<'a>(engine: &'a Engine) -> &'a Self {
        &engine.window
    }

    fn get_mut<'a>(engine: &'a mut Engine) -> &'a mut Self {
        &mut engine.window
    }
}

impl Engine {
    pub fn new() -> Self {
        #[cfg(feature = "profiler")]
        let perf = crate::utils::scoped_perf::ScopedPerf::new();

        // init window
        let window = {
            let sdl_context = sdl2::init().unwrap();
            let sdl_video = sdl_context.video().unwrap();
            let sdl_window = sdl_video
                .window("rust engine", 800, 600)
                .opengl()
                .resizable()
                .position_centered()
                .build()
                .unwrap();
            sdl_video.gl_attr().set_context_flags().debug().set(); // for debug with RenderDoc
            let gl_context = sdl_window.gl_create_context().unwrap();
            sdl_video.gl_set_swap_interval(1).unwrap(); // vsync on
            gl::load_with(|s| sdl_video.gl_get_proc_address(s) as *const std::os::raw::c_void);

            unsafe { gl::ClearColor(0.3, 0.3, 0.5, 1.0) };

            unsafe {
                println!(
                    "VERSION: {}",
                    utils::c_string_to_str(gl::GetString(gl::VERSION))
                );
            }

            Window {
                sdl_context,
                sdl_video,
                sdl_window,
                gl_context,
            }
        };

        // init input
        let input = Input::new(window.sdl_context.event_pump().unwrap());

        // init time
        let time = Time::new(window.sdl_context.timer().unwrap());

        // init ecs

        Self {
            world: World::new(),
            exit_flag: false,
            logics: Vec::new(),
            window,
            input,
            time,
        }
    }

    pub fn get_subsystem<T: EngineSubsystem>(&self) -> &T {
        T::get(self)
    }

    pub fn get_subsystem_mut<T: EngineSubsystem>(&mut self) -> &mut T {
        T::get_mut(self)
    }

    pub fn add_logic<T: StateObject>(&mut self, logic: StateLogic<T>) {
        self.logics.push(Box::new(logic));
    }

    pub fn run(&mut self) {
        self.init();
        self.time.update();
        while !self.exit_flag {
            self.time.update();

            self.poll_events();
            self.update();
            self.post_update();
            self.render();
            self.swap();
        }
        self.shutdown();
    }

    fn init(&mut self) {
        self.run_logic_function(LogicFuncType::Init);
    }

    fn shutdown(&mut self) {
        self.run_logic_function(LogicFuncType::Shutdown);
    }

    fn poll_events(&mut self) {
        // TODO: to input
        for event in self.input.get_event_pump_mut().poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => self.exit_flag = true,
                _ => {}
            }
        }

        self.input.update();
    }

    fn update(&mut self) {
        self.run_logic_function(LogicFuncType::Update);
    }

    fn post_update(&mut self) {
        self.run_logic_function(LogicFuncType::PostUpdate);
    }

    fn render(&mut self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        self.run_logic_function(LogicFuncType::Render);
    }

    fn swap(&mut self) {
        self.run_logic_function(LogicFuncType::Swap);
        self.window.sdl_window.gl_swap_window();
    }

    fn run_logic_function(&mut self, func_type: LogicFuncType) {
        // TODO: shit?
        let mut systems = std::mem::take(&mut self.logics);
        for system in &mut systems {
            let mut engine_interface = EngineInterface::new(self);
            let mut commands = Commands::new();
            system.run(&self.world, func_type, &engine_interface, &mut commands);
            self.execute_commands(commands.commands);
        }
        self.logics = systems;
    }

    fn execute_commands(&mut self, commands: Vec<Command>) {
        for command in commands {
            self.execute_command(command);
        }
    }

    fn execute_command(&mut self, command: Command) {
        match command {
            Command::Exit => {
                self.exit_flag = true;
            }
        }
    }
}
