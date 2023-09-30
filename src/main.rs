mod ecs;
mod math;
mod utils;

use crate::ecs::*;
use crate::math::*;
use crate::utils::combo_iterator::*;
use std::collections::HashMap;
use std::iter::Zip;

#[derive(Clone, Copy, Default, Debug)]
struct Position(Vec3f);

#[derive(Clone, Copy, Default, Debug)]
struct Mass(f32);

#[derive(Clone, Copy, Default, Debug)]
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
    let mut world = World::new();
    world.register_component::<Position>();
    world.register_component::<Mass>();

    world.get_component_array::<Position>().unwrap();
    world.get_component_array::<Mass>().unwrap();
    // world.get_component_array::<Velocity>().unwrap();


    world.query_mut::<&Position>();

    // let mut ecs = Ecs::new();
    //
    // ecs.register_component::<Position>();
    // ecs.register_component::<Mass>();
    //
    // {
    //     let e = ecs.create_entity();
    //     ecs.add_component(Position::default(), e);
    // }
    // {
    //     let e = ecs.create_entity();
    //     ecs.add_component(Position::default(), e);
    //     ecs.add_component(Mass::default(), e);
    // }
    // {
    //     let e = ecs.create_entity();
    //     ecs.add_component(Mass::default(), e);
    // }
    // {
    //     let e = ecs.create_entity();
    //     ecs.add_component(Position::default(), e);
    //     ecs.add_component(Mass::default(), e);
    // }
    // {
    //     let e = ecs.create_entity();
    // }
    // {
    //     let e = ecs.create_entity();
    //     ecs.add_component(Position::default(), e);
    // }
    //
    // // let q = ecs.query::<(Position, Mass)>();
    // // for comps in q.iter()
    // // {
    // //     println!("ent: {:?}| pos: {:?} | mass: {:?}", comps.0, comps.1, comps.2);
    // // }
    //
    // // let q = ecs.query::<Position>();
    // // for a in q.iterate() {
    // //     println!("ent: {:?} | pos: {:?}", a.0, a.1);
    // // }
}
