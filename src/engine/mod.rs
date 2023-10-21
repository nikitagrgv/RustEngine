mod engine_subsystem;

use crate::ecs;
use crate::ecs::World;
use crate::engine::engine_subsystem::EngineSubsystem;
use crate::input::*;
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};

pub enum Command {
    Exit,
}

pub struct EngineInterface<'a> {
    commands: Vec<Command>,
    engine: &'a mut Engine,
}

impl<'a> EngineInterface<'a> {
    pub fn new(engine: &'a mut Engine) -> Self {
        Self {
            commands: Vec::new(),
            engine,
        }
    }

    pub fn add(&mut self, command: Command) {
        self.commands.push(command);
    }

    pub fn get_subsystem<T: EngineSubsystem>(&mut self) -> &T {
        self.engine.get_subsystem()
    }

    pub fn get_subsystem_mut<T: EngineSubsystem>(&mut self) -> &mut T {
        self.engine.get_subsystem_mut()
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum LogicFuncType {
    Init,
    Update,
    PostUpdate,
    Render,
    Swap,
    Shutdown,
}

enum LogicFuncVariant<T: StateObject> {
    Default(fn(&mut T, &mut EngineInterface)),
}

struct LogicFunc<T: StateObject> {
    func_type: LogicFuncType,
    function: LogicFuncVariant<T>,
}

pub trait StateObject: 'static {}

impl<T: 'static> StateObject for T {}

pub trait Logic {
    fn run(&mut self, func_type: LogicFuncType, commands: &mut EngineInterface);
}

pub struct StateLogic<T: StateObject> {
    object: T,
    functions: Vec<LogicFunc<T>>,
}

impl<T: StateObject> StateLogic<T> {
    pub fn new(object: T) -> Self {
        Self {
            object,
            functions: Vec::new(),
        }
    }

    pub fn add_function(
        &mut self,
        function: fn(&mut T, &mut EngineInterface),
        func_type: LogicFuncType,
    ) {
        let lf = LogicFunc {
            func_type,
            function: LogicFuncVariant::Default(function),
        };
        self.functions.push(lf);
    }
}

impl<T: StateObject> Logic for StateLogic<T> {
    fn run(&mut self, func_type: LogicFuncType, commands: &mut EngineInterface) {
        for function in &self.functions {
            if function.func_type == func_type {
                match function.function {
                    LogicFuncVariant::Default(f) => {
                        f(&mut self.object, commands);
                    }
                }
            }
        }
    }
}

pub struct Window {
    sdl_context: sdl2::Sdl,
    sdl_video: sdl2::VideoSubsystem,
    sdl_canvas: sdl2::render::WindowCanvas,
}

pub struct Engine {
    world: World,
    exit_flag: bool,
    systems: Vec<Box<dyn Logic>>,
    window: Window,
    input: Input,
}

impl EngineSubsystem for Input {
    fn get<'a>(engine: &'a Engine) -> &'a Self {
        &engine.input
    }

    fn get_mut<'a>(engine: &'a mut Engine) -> &'a mut Self {
        &mut engine.input
    }
}

impl Engine {
    pub fn new() -> Self {
        let window = {
            let sdl_context = sdl2::init().unwrap();
            let sdl_video = sdl_context.video().unwrap();
            let sdl_window = sdl_video
                .window("rust engine", 800, 600)
                .position_centered()
                .build()
                .unwrap();
            let mut sdl_canvas = sdl_window.into_canvas().build().unwrap();
            sdl_canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 255, 255));
            sdl_canvas.clear();
            sdl_canvas.present();

            Window {
                sdl_context,
                sdl_video,
                sdl_canvas,
            }
        };

        let input = Input::new(window.sdl_context.event_pump().unwrap());

        Self {
            world: World::new(),
            exit_flag: false,
            systems: Vec::new(),
            window,
            input,
        }
    }

    pub fn get_subsystem<T: EngineSubsystem>(&self) -> &T {
        T::get(self)
    }

    pub fn get_subsystem_mut<T: EngineSubsystem>(&mut self) -> &mut T {
        T::get_mut(self)
    }

    pub fn add_logic<T: StateObject>(&mut self, logic: StateLogic<T>) {
        self.systems.push(Box::new(logic));
    }

    pub fn run(&mut self) {
        self.init();
        while !self.exit_flag {
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
        // let keys_state = sdl2::keyboard::KeyboardState::new(&self.window.sdl_event_pump);
        // if keys_state.is_scancode_pressed(Scancode::T) {
        //     println!("T PRESSED!!!!!!!!!");
        // }
    }

    fn update(&mut self) {
        self.run_logic_function(LogicFuncType::Update);
    }

    fn post_update(&mut self) {
        self.run_logic_function(LogicFuncType::PostUpdate);
    }

    fn render(&mut self) {
        self.window.sdl_canvas.clear();
        self.run_logic_function(LogicFuncType::Render);
    }

    fn swap(&mut self) {
        self.run_logic_function(LogicFuncType::Swap);
        self.window.sdl_canvas.present();
    }

    fn run_logic_function(&mut self, func_type: LogicFuncType) {
        // TODO: shit?
        let mut systems = std::mem::take(&mut self.systems);
        for system in &mut systems {
            let mut engine_interface = EngineInterface::new(self);
            system.run(func_type, &mut engine_interface);
            let mut commands = engine_interface.commands;
            self.execute_commands(commands);
        }
        self.systems = systems;
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
