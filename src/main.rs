mod ecs;
mod math;
mod utils;

use crate::ecs::*;
use crate::math::*;
use crate::utils::combo_iterator::*;
use std::collections::HashMap;
use std::iter::Zip;

#[derive(Clone, Copy, Default)]
struct Position(Vec3f);

#[derive(Clone, Copy, Default)]
struct Mass(f32);

#[derive(Clone, Copy, Default)]
struct Velocity(Vec3f);

// use bevy::prelude::{*};
// fn greet_people(query: Query<&Name, With<Person>>) {
//     for name in &query {
//         println!("hello {}!", name.0);
//     }
// }

// trait Query
// {
//     type Item;
//
//     fn fetch() -> Self::Item;
// }

use std::cell::Ref;
use std::cell::RefCell;
use std::cell::RefMut;
// use core::cell::RefCell;
use bevy::prelude::Reflect;
use bevy::utils::tracing::instrument::WithSubscriber;

struct G(pub i32);

struct St {
    pub val: G,
}

fn gett(c: &RefCell<St>) -> RefMut<G> {
    let r = c.borrow_mut();
    let ff = std::cell::RefMut::map(r, |a: &mut St| &mut a.val);
    ff
}

fn main() {
    let mut ecs = Ecs::new();

    ecs.register_component::<Position>();
    ecs.register_component::<Mass>();

    {
        let e = ecs.create_entity();
        ecs.add_component(Position::default(), e);
    }
    {
        let e = ecs.create_entity();
        ecs.add_component(Position::default(), e);
    }
    {
        let e = ecs.create_entity();
        ecs.add_component(Position::default(), e);
    }
    {
        let e = ecs.create_entity();
        ecs.add_component(Position::default(), e);
    }
    {
        let e = ecs.create_entity();
        ecs.add_component(Position::default(), e);
    }
    {
        let e = ecs.create_entity();
        ecs.add_component(Position::default(), e);
    }

    let q = ecs.query::<Position>();
    for a in q.iterate()
    {

    }
}
