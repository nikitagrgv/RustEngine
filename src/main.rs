mod ecs;
mod math;
mod utils;

use crate::ecs::*;
use crate::math::*;

#[derive(Clone, Copy, Default, Debug)]
struct Position(Vec3f);

impl Position {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(Vec3f::new(x, y, z))
    }
}

#[derive(Clone, Copy, Default, Debug)]
struct Mass(f32);

impl Mass {
    pub fn new(mass: f32) -> Self {
        Self(mass)
    }
}

#[derive(Clone, Copy, Default, Debug)]
struct Velocity(Vec3f);

impl Velocity {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(Vec3f::new(x, y, z))
    }
}

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



fn main() {
    // let mut num = 123i32;
    // struct Spam<'w>{
    //     m: &'w mut i32,
    // }
    // // not 'w: 'a !!!!!!
    // fn ff<'w, 'a: 'w>(q: &'w mut Spam<'a>) {
    //
    // }
    //
    // let mut s = Spam{m: &mut num};
    // ff(&mut s);
    // ff(&mut s);


    let mut world = World::new();
    world.register_component::<Position>();
    world.register_component::<Mass>();

    let e0 = world.create_entity();
    let e1 = world.create_entity();
    let e2 = world.create_entity();

    world.set_component(Position::new(1.0, 2.0, 3.0), e0);
    world.set_component(Mass::new(4.0), e0);

    world.set_component(Position::new(7.0, 8.0, 9.0), e1);
    world.set_component(Mass::new(10.0), e1);

    // world.add_component(Position::new(7.0, 8.0, 9.0), e2);
    world.set_component(Mass::new(110.0), e2);

    // fn fff<'a, 'w: 'a>(q: &'a mut Query<'w, (&Position)>, e: Entity) {
    //     q.fetch_entity(e);
    // }
    //
    // let mut q = world.query::<(&Position)>();
    // fff(&mut q, e0);
    // fff(&mut q, e0);
    //

    let mut q = world.query::<(&Position, &Mass)>();
    if let FetchResult::Some(cmps) = q.fetch_entity(e0)
    {
        println!("FOUND FOR e0");
    }
    if let FetchResult::Some(cmps) = q.fetch_entity(e1)
    {
        println!("FOUND FOR e1");
    }
    if let FetchResult::Some(cmps) = q.fetch_entity(e2)
    {
        println!("FOUND FOR e2");
    }
    drop(q);

    let mut q = world.query::<(&mut Position, &Mass)>();
    if let FetchResult::Some(cmps) = q.fetch_entity(e0)
    {
        println!("FOUND FOR e0");
    }
    if let FetchResult::Some(cmps) = q.fetch_entity(e1)
    {
        println!("FOUND FOR e1");
    }
    if let FetchResult::Some(cmps) = q.fetch_entity(e2)
    {
        println!("FOUND FOR e2");
    }

    // let qit = q.iter();
    // for a in qit
    // {
    //     println!("Afafafa: {:?}", a);
    // }

    let qitm = q.iter_mut();
    for a in qitm
    {
        a.0.0.x = 111111f32;
    }

    let qitm = q.iter_mut();
    for a in qitm
    {
        a.0.0.x = 161111f32;
    }


    let qit = q.iter();
    for a in qit
    {
        println!("Afafafa: {:?}", a);
    }
    // let mm = HashMap::<i32,i32>::new();
    // mm.iter_mut()

    // fff(&mut q, e0);
    // fff(&mut q, e1);


    //
    // fn ttt<'w>(vec: &'w mut Vec<i32>) {
    //     println!("{:#?}", vec);
    // }
    //
    // let mut s: Vec<i32> = Vec::<i32>::new();
    // s.iter_mut()
    // s.push(123);
    // s.push(135);
    // s.push(616);
    //
    // ttt(&mut s);
    // ttt(&mut s);
    //
    // if let Some(a) = s.as_mut_slice().get_mut(0) {
    //     println!("{:#?}", a);
    // }
    // if let Some(a) = s.as_mut_slice().get_mut(1) {
    //     println!("{:#?}", a);
    // }
}
