mod ecs;

use ecs::*;
use std::collections::HashMap;

struct Position {}

struct Mass {}

struct Velocity {}

struct PhysicsSystem {}

fn main() {
    let mut ecs = Ecs::new();
    let e0 = ecs.create_entity();

    ecs.register_component::<Position>();
    ecs.register_component::<Mass>();

    ecs.add_component(e0, Position {});
    let p = ecs.get_component::<Position>(e0).unwrap();

}
