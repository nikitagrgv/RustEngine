use crate::ecs;
use crate::ecs::World;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub enum Command {
    Exit,
}

pub struct Commands {
    commands: Vec<Command>,
}

impl Commands {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    pub fn add(&mut self, command: Command) {
        self.commands.push(command);
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
    Default(fn(&mut T, &mut Commands)),
}

struct LogicFunc<T: StateObject> {
    func_type: LogicFuncType,
    function: LogicFuncVariant<T>,
}

pub trait StateObject: 'static {}

impl<T: 'static> StateObject for T {}

pub trait Logic {
    fn run(&mut self, func_type: LogicFuncType, commands: &mut Commands);
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

    pub fn add_function(&mut self, function: fn(&mut T, &mut Commands), func_type: LogicFuncType) {
        let lf = LogicFunc {
            func_type,
            function: LogicFuncVariant::Default(function),
        };
        self.functions.push(lf);
    }
}

impl<T: StateObject> Logic for StateLogic<T> {
    fn run(&mut self, func_type: LogicFuncType, commands: &mut Commands) {
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
    sdl_event_pump: sdl2::EventPump,
}

pub struct Engine {
    world: World,
    exit_flag: bool,
    systems: Vec<Box<dyn Logic>>,
    window: Window,
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
            let mut sdl_event_pump = sdl_context.event_pump().unwrap();

            Window {
                sdl_context,
                sdl_video,
                sdl_canvas,
                sdl_event_pump,
            }
        };

        Self {
            world: World::new(),
            exit_flag: false,
            systems: Vec::new(),
            window,
        }
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
        for event in self.window.sdl_event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => self.exit_flag = true,
                _ => {}
            }
        }
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
            let mut commands = Commands::new();
            system.run(func_type, &mut commands);
            self.execute_commands(commands);
        }
        self.systems = systems;
    }

    fn execute_commands(&mut self, commands: Commands) {
        for command in commands.commands {
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
