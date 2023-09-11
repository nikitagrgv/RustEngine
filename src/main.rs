mod ecs;
mod math;
mod utils;

use std::ops::DerefMut;
use crate::ecs::*;
use crate::math::*;

#[derive(Clone, Copy, Default)]
struct Position(Vec3f);

#[derive(Clone, Copy, Default)]
struct Mass(f32);

#[derive(Clone, Copy, Default)]
struct Velocity(Vec3f);


fn main() {
    // Set up
    let mut ecs = Ecs::new();

    ecs.register_component::<Position>()
        .register_component::<Velocity>()
        .register_component::<Mass>();

    let phys_sys = {
        let mut phys_sig = Signature::new();
        phys_sig
            .add_component::<Position>()
            .add_component::<Velocity>()
            .add_component::<Mass>();
        ecs.create_system_with_signature(phys_sig)
    };

    fn create_phys_entity(ecs: &mut Ecs) -> Entity {
        let e = ecs.create_entity();
        ecs.add_component(e, Position::default());
        ecs.add_component(e, Velocity::default());
        ecs.add_component(e, Mass::default());
        e
    }

    let e0 = ecs.create_entity();
    let e1 = ecs.create_entity();
    let e2 = ecs.create_entity();
    let e3 = ecs.create_entity();
    let e4 = ecs.create_entity();

    // Using ecs
    for _ in [0..10]
    {
        let phys_ents = ecs.get_system_entities(phys_sys);
        println!("PHYS ENTS {:#?}", phys_ents);
        for &ent in phys_ents {
            let mut pos = ecs.get_component_mut::<Position>(ent);
            let mut vel = ecs.get_component_mut::<Velocity>(ent);
            let mut mass = ecs.get_component_mut::<Mass>(ent);

            pos.0.x = pos.0.x + 1f32;

            println!("ENTITY {:#?} - POS {:#?}", ent, pos.0.x);
        }
    }

}
