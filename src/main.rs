mod ecs;
mod math;
mod utils;

use crate::ecs::*;
use crate::math::*;
use std::ops::{Deref, DerefMut};
use crate::utils::to_any::ToAny;

#[derive(Clone, Copy, Default)]
struct Position(Vec3f);

#[derive(Clone, Copy, Default)]
struct Mass(f32);

#[derive(Clone, Copy, Default)]
struct Velocity(Vec3f);





// impl<'a, C: 'static> Deref for ComponentRef<'a, C> {
//     type Target = C;
//
//     fn deref(&self) -> &Self::Target {
//         // &self.comp
//         todo!()
//     }
// }
//
// impl<'a, C: 'static> DerefMut for ComponentRef<'a, C> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         todo!()
//
//         // &mut self.comp
//     }
// }
//



fn main() {
    // Set up
    let mut ecs = Ecs::new();

    ecs.register_component::<Position>()
        .register_component::<Velocity>()
        .register_component::<Mass>();

    let phys_sys = {
        let mut phys_sig = Signature::new();
        phys_sig.add::<Position>().add::<Velocity>().add::<Mass>();
        ecs.create_system_with_signature(phys_sig)
    };

    fn create_phys_entity(ecs: &mut Ecs) -> Entity {
        let e = ecs.create_entity();
        ecs.add_component(e, Position::default());
        ecs.add_component(e, Velocity::default());
        ecs.add_component(e, Mass::default());
        e
    }

    let e0 = create_phys_entity(&mut ecs);
    let e1 = create_phys_entity(&mut ecs);
    let e2 = create_phys_entity(&mut ecs);
    let e3 = create_phys_entity(&mut ecs);
    let e4 = create_phys_entity(&mut ecs);

    // Using ecs
    for _ in [0..10] {
        let phys_ents = ecs.get_system_entities(phys_sys);
        println!("PHYS ENTS {:?}", phys_ents);
        for &ent in phys_ents {
            let mut pos = ecs.get_component_mut::<Position>(ent);
            let mut vel = ecs.get_component_mut::<Velocity>(ent);
            let mut mass = ecs.get_component_mut::<Mass>(ent);

            pos.0.x = pos.0.x + 1f32;

            // println!("ENTITY {:?} - POS {:?}", ent, pos.0.x);
        }
    }
}
