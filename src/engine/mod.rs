use crate::ecs;
use crate::ecs::World;

#[derive(Eq, PartialEq)]
pub enum LogicFuncType {
    Update,
    PostUpdate,
    Render,
    Swap,
}

enum LogicFuncVariant<T: StateObject> {
    Default(fn(&mut T)),
}

struct LogicFunc<T: StateObject> {
    func_type: LogicFuncType,
    function: LogicFuncVariant<T>,
}

pub trait StateObject: 'static {}

impl<T: 'static> StateObject for T {}

pub trait Logic {
    fn run(&mut self, func_type: LogicFuncType);
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

    pub fn add_function(&mut self, function: fn(&mut T), func_type: LogicFuncType) {
        let lf = LogicFunc {
            func_type,
            function: LogicFuncVariant::Default(function),
        };
        self.functions.push(lf);
    }
}

impl<T: StateObject> Logic for StateLogic<T> {
    fn run(&mut self, func_type: LogicFuncType) {
        for function in &self.functions {
            if function.func_type == func_type {
                match function.function {
                    LogicFuncVariant::Default(f) => {
                        f(&mut self.object);
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

    // pub fn add_system<S: System>(&mut self, system: S) {
    //     self.systems.push(Box::new(system))
    // }

    pub fn run(&mut self) {
        while !self.exit_flag {
            self.update();
            self.post_update();
            self.render();
            self.swap();
        }
    }

    fn update(&mut self) {
        for system in &mut self.systems {
            system.run(LogicFuncType::Update);
        }
    }

    fn post_update(&mut self) {
        for system in &mut self.systems {
            system.run(LogicFuncType::PostUpdate);
        }
    }

    fn render(&mut self) {
        for system in &mut self.systems {
            system.run(LogicFuncType::Render);
        }
    }

    fn swap(&mut self) {
        for mut system in &mut self.systems {
            system.run(LogicFuncType::Swap);
        }
    }
}
