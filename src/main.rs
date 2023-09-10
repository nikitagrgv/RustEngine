mod ecs;
mod utils;

use ecs::*;

struct Position {}

struct Mass {}

struct Velocity {}

struct PhysicsSystem {}

fn main() {
    let mut em = EntityManager::new();
    let e = em.create_entity();

    let mut cm = ComponentManager::new();

    cm.register_component::<Mass>();
    cm.add_component(e, Mass {});
    let m = cm.get_component::<Mass>(e);
    let m = cm.get_component_mut::<Mass>(e);

    let mut sm = SystemManager::new();
    let sys = sm.create_system();


}
