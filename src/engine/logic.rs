use crate::ecs::*;
use crate::engine::{Commands, EngineInterface};

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
    fn call(&self, state: &mut T, world: &World, ei: &EngineInterface, commands: &mut Commands);
}

struct EcsFunctionT<T: StateObject, F: Fetcherable> {
    func: fn(&mut T, Query<'_, F>, &EngineInterface, &mut Commands),
}

impl<T: StateObject, F: Fetcherable> EcsFunction<T> for EcsFunctionT<T, F> {
    fn call(&self, state: &mut T, world: &World, ei: &EngineInterface, commands: &mut Commands) {
        let q = world.query::<F>();
        (self.func)(state, q, ei, commands);
    }
}

enum LogicFuncVariant<T: StateObject> {
    Default(fn(&mut T, &EngineInterface, &mut Commands)),
    Ecs(Box<dyn EcsFunction<T>>),
}

struct LogicFunc<T: StateObject> {
    func_type: LogicFuncType,
    function: LogicFuncVariant<T>,
}

pub trait StateObject: 'static {}

impl<T: 'static> StateObject for T {}

pub trait Logic {
    fn run(
        &mut self,
        world: &World,
        func_type: LogicFuncType,
        ei: &EngineInterface,
        commands: &mut Commands,
    );
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
        function: fn(&mut T, &EngineInterface, &mut Commands),
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
        function: fn(&mut T, query: Query<F>, &EngineInterface, &mut Commands),
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
    fn run(
        &mut self,
        world: &World,
        func_type: LogicFuncType,
        ei: &EngineInterface,
        commands: &mut Commands,
    ) {
        for function in &self.functions {
            if function.func_type == func_type {
                match &function.function {
                    LogicFuncVariant::Default(f) => {
                        f(&mut self.object, ei, commands);
                    }
                    LogicFuncVariant::Ecs(f) => f.call(&mut self.object, &world, ei, commands),
                }
            }
        }
    }
}
