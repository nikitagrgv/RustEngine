use crate::engine::engine_subsystem::EngineSubsystem;
use crate::engine::{Commands, Engine, EngineInterface, Window};
use crate::world::*;

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

//     state: &mut GravitySystemState,
//     mut query: Query<(&mut Position, &mut Velocity, &Mass)>,
//     ei: &EngineInterface,
//     commands: &mut Commands,

pub trait LogicFunc<'a> {
    fn call(&self, engine: &'a mut Engine);
}

pub trait LogicFuncArgument<'a> {
    type Holder;
    fn fetch(engine: &'a Engine) -> Self::Holder;
    fn unpack(holder: &'a mut Self::Holder) -> Self;
}

impl<'a, T: StateObject> LogicFuncArgument<'a> for &T {
    type Holder = Self;
    fn fetch(engine: &Engine) -> Self::Holder {
        todo!()
    }

    fn unpack(holder: &mut Self::Holder) -> Self {
        todo!()
    }
}

impl<'a, T: StateObject> LogicFuncArgument<'a> for &mut T {
    type Holder = ();

    fn fetch(engine: &Engine) -> Self::Holder {
        todo!()
    }

    fn unpack(holder: &mut Self::Holder) -> Self {
        todo!()
    }
}

impl<'a, T: Fetcherable> LogicFuncArgument<'a> for &mut Query<'a, T> {
    type Holder = Query<'a, T>;

    fn fetch(engine: &'a Engine) -> Self::Holder {
        engine.world.query::<T>()
    }

    fn unpack(holder: &'a mut Self::Holder) -> Self {
        holder
    }
}

impl<'a> LogicFuncArgument<'a> for &mut Commands {
    type Holder = ();

    fn fetch(engine: &Engine) -> Self::Holder {
        todo!()
    }

    fn unpack(holder: &mut Self::Holder) -> Self {
        todo!()
    }
}

// impl LogicFuncArgument for &Window {
//     type Holder = &Window;
//
//     fn fetch(engine: & Engine) -> Self::Holder {
//         &engine.window
//     }
//
//     fn unpack(holder: &mut Self::Holder) -> Self {
//         todo!()
//     }
// }

impl<'a, T0: LogicFuncArgument<'a> + 'a> LogicFunc<'a> for fn(T0) {
    fn call(&self, engine: &'a mut Engine) {
        let mut h0 = T0::fetch(engine);
        self(T0::unpack(&mut h0));
    }
}

// impl<'a, T0: LogicFuncArgument<'a>, T1: LogicFuncArgument<'a>> LogicFunc<'a> for fn(T0, T1) {
//     fn call(&self, engine: &'a mut Engine) {
//         let mut h0 = T0::fetch(engine);
//         let mut h1 = T1::fetch(engine);
//         self(T0::unpack(&mut h0), T1::unpack(&mut h1));
//     }
// }

enum LogicFuncVariant<T: StateObject> {
    Default(fn(&mut T, &EngineInterface, &mut Commands)),
    Ecs(Box<dyn EcsFunction<T>>),
}

struct LogicFuncWithType<T: StateObject> {
    func_type: LogicFuncType,
    function: LogicFuncVariant<T>,
}

pub trait StateObject: 'static {}

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
    functions: Vec<LogicFuncWithType<T>>,
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
        let lf = LogicFuncWithType {
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
        let lf = LogicFuncWithType {
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
