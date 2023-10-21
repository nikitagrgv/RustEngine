use crate::engine::EngineInterface;

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
