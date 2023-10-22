use crate::ecs::*;
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

trait EcsFunction<T: StateObject> {
    fn call(&self, state: &mut T, world: &World, ei: &mut EngineInterface);
}

struct EcsFunctionT<T: StateObject, F: Fetcherable> {
    func: fn(&mut T, Query<'_, F>, &mut EngineInterface),
}

impl<T: StateObject, F: Fetcherable> EcsFunction<T> for EcsFunctionT<T, F> {
    fn call(&self, state: &mut T, world: &World, ei: &mut EngineInterface) {
        let q = world.query::<F>();
        (self.func)(state, q, ei);
    }
}

enum LogicFuncVariant<T: StateObject> {
    Default(fn(&mut T, &mut EngineInterface)),
    Ecs(Box<dyn EcsFunction<T>>),
}

struct LogicFunc<T: StateObject> {
    func_type: LogicFuncType,
    function: LogicFuncVariant<T>,
}

pub trait StateObject: 'static {}

impl<T: 'static> StateObject for T {}

pub trait Logic {
    fn run(&mut self, world: &World, func_type: LogicFuncType, commands: &mut EngineInterface);
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

    pub fn add_ecs_function<F: Fetcherable + 'static>(
        &mut self,
        function: fn(&mut T, query: Query<F>, &mut EngineInterface),
        func_type: LogicFuncType,
    ) {
        let lf = LogicFunc {
            func_type,
            function: LogicFuncVariant::Ecs(Box::new(EcsFunctionT { func: function })),
        };
        self.functions.push(lf);
    }
}

impl<T: StateObject> Logic for StateLogic<T> {
    fn run(&mut self, world: &World, func_type: LogicFuncType, ei: &mut EngineInterface) {
        for function in &self.functions {
            if function.func_type == func_type {
                match &function.function {
                    LogicFuncVariant::Default(f) => {
                        f(&mut self.object, ei);
                    }
                    LogicFuncVariant::Ecs(f) => f.call(&mut self.object, &world, ei),
                }
            }
        }
    }
}
