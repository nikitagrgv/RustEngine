use crate::ecs;
use crate::ecs::World;

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
    Shutdown
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

pub struct Engine {
    world: World,
    exit_flag: bool,
    systems: Vec<Box<dyn Logic>>,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            world: World::new(),
            exit_flag: false,
            systems: Vec::new(),
        }
    }

    pub fn add_logic<T: StateObject>(&mut self, logic: StateLogic<T>) {
        self.systems.push(Box::new(logic));
    }

    pub fn run(&mut self) {
        self.init();
        while !self.exit_flag {
            self.update();
            self.post_update();
            self.render();
            self.swap();
        }
        self.shutdown();
    }

    fn init(&mut self)
    {
        self.run_logic_function(LogicFuncType::Init);
    }

    fn shutdown(&mut self)
    {
        self.run_logic_function(LogicFuncType::Shutdown);
    }

    fn update(&mut self) {
        self.run_logic_function(LogicFuncType::Update);
    }

    fn post_update(&mut self) {
        self.run_logic_function(LogicFuncType::PostUpdate);
    }

    fn render(&mut self) {
        self.run_logic_function(LogicFuncType::Render);
    }

    fn swap(&mut self) {
        self.run_logic_function(LogicFuncType::Swap);
    }

    fn run_logic_function(&mut self, func_type: LogicFuncType) {
        for system in &mut self.systems {
            let mut commands = Commands::new();
            system.run(func_type, &mut commands);
            self.execute_commands(commands);
        }
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
