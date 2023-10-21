use crate::engine::Engine;

pub trait EngineSubsystem {
    fn get<'a>(engine: &'a Engine) -> &'a Self;
    fn get_mut<'a>(engine: &'a mut Engine) -> &'a mut Self;
}
