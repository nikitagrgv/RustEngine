use std::any::{Any, TypeId};
use std::cell::{Ref, RefCell, RefMut};
use std::collections::{HashMap, HashSet};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut, Index};

/// Entity is just id. You can assign components to Entity
#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
pub struct Entity(usize);

trait ComponentArray {
    fn push_none(&mut self);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub struct ComponentArrayTemplate<T: 'static> {
    components: Vec<Option<T>>,
}

impl<T: 'static> ComponentArrayTemplate<T> {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }
}

impl<T> ComponentArray for ComponentArrayTemplate<T> {
    fn push_none(&mut self) {
        self.components.push(None);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub struct Ecs {
    entities_count: usize,
    // TODO: HashMap
    component_arrays: Vec<RefCell<Box<dyn ComponentArray>>>,
}

impl Ecs {
    pub fn new() -> Self {
        Self {
            entities_count: 0,
            component_arrays: Vec::new(),
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        let ent_id = self.entities_count;
        self.entities_count += 1;
        for component_array in &mut self.component_arrays {
            component_array.borrow_mut().push_none();
        }
        Entity(ent_id)
    }

    pub fn register_component<T: 'static>(&mut self) {
        assert!(self.get_component_array::<T>().is_none());
        let mut ca = Box::new(ComponentArrayTemplate::<T>::new());
        for _ in 0..self.entities_count {
            ca.push_none();
        }
        self.component_arrays.push(RefCell::new(ca));
    }

    pub fn add_component<T: 'static>(&mut self, component: T, e: Entity) {
        self.get_component_array_mut::<T>()
            .expect("Component is not registered")
            .components[e.0] = Some(component);
    }

    pub fn remove_component<T: 'static>(&mut self, component: T, e: Entity) {
        self.get_component_array_mut::<T>()
            .expect("Component is not registered")
            .components[e.0] = None;
    }

    pub fn get_component_array<T: 'static>(&self) -> Option<Ref<ComponentArrayTemplate<T>>> {
        // TODO: wtf is this shittttt?
        for c in self.component_arrays.iter() {
            let is_such = c
                .borrow()
                .deref()
                .as_any()
                .downcast_ref::<ComponentArrayTemplate<T>>()
                .is_some();
            let comp_ref = c.borrow();
            if is_such {
                let ret_ref = Ref::map(comp_ref, |comp| {
                    comp.as_any()
                        .downcast_ref::<ComponentArrayTemplate<T>>()
                        .unwrap()
                });
                return Some(ret_ref);
            }
        }
        None
    }

    pub fn get_component_array_mut<T: 'static>(&self) -> Option<RefMut<ComponentArrayTemplate<T>>> {
        // TODO: wtf is this shittttt?
        for c in self.component_arrays.iter() {
            let is_such = c
                .borrow()
                .deref()
                .as_any()
                .downcast_ref::<ComponentArrayTemplate<T>>()
                .is_some();
            let comp_ref = c.borrow_mut();
            if is_such {
                let ret_ref = RefMut::map(comp_ref, |comp| {
                    comp.as_any_mut()
                        .downcast_mut::<ComponentArrayTemplate<T>>()
                        .unwrap()
                });
                return Some(ret_ref);
            }
        }
        None
    }

    pub fn query<T: 'static>(&self) -> Query<T> {
        Query::new(self.get_component_array().unwrap())
    }
}

pub struct Query<'a, T: 'static> {
    component_array_ref: Ref<'a, ComponentArrayTemplate<T>>,
}

impl<'a, T: 'static> Query<'a, T> {
    pub fn new(component_array_ref: Ref<'a, ComponentArrayTemplate<T>>) -> Self {
        Self {
            component_array_ref,
        }
    }

    pub fn iterate(&'a self) -> ComponentIterator<'a, T> {
        ComponentIterator::new(self)
    }
}

pub struct ComponentIterator<'a, T: 'a + 'static> {
    query: &'a Query<'a, T>,
    cur_entity: Entity,
}

impl<'a, T: 'static> ComponentIterator<'a, T> {
    pub fn new(query: &'a Query<'a, T>) -> Self {
        Self {
            query,
            cur_entity: Entity(0),
        }
    }
}

impl<'a, T: 'a + 'static> Iterator for ComponentIterator<'a, T> {
    type Item = (Entity, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let ca = &self.query.component_array_ref.components;
        loop {
            let cur_entity = self.cur_entity;
            self.cur_entity.0 += 1;
            let comp = ca.get(cur_entity.0);
            match comp {
                None => {
                    break None;
                }
                Some(Some(comp)) => {
                    break Some((cur_entity, comp));
                }
                Some(None) => {}
            }
        }
    }
}

// pub struct Fetch<'a, T: 'static>
// {
//     component_array: &'a ComponentArrayTemplate<T>,
// }
//
//
// ($($name::Fetch<'w>,)*);

// trait Fetch<'a> {
//     type ComponentArrays;
//     type Items;
//
//     fn init_fetch(ecs: &Ecs) -> Self::ComponentArrays;
//     fn fetch(ca: &Self::ComponentArrays, e: Entity) -> &Option<Self::Items>;
// }
//
// impl<'a, T: 'static> Fetch<'a> for (T, ) {
//     type ComponentArrays = Ref<'a, ComponentArrayTemplate<T>>;
//     type Items = T;
//
//     fn init_fetch(ecs: &Ecs) -> Self::ComponentArrays {
//         ecs.get_component_array::<T>().unwrap()
//     }
//
//     fn fetch(ca: &Self::ComponentArrays, e: Entity) -> &Option<Self::Items> {
//         &ca.components[e.0]
//     }
// }
//
// macro_rules! impl_fetch {
//     ($($type_name: ident, $name: ident),*) => {
//         impl<'a, $($type_name: 'static,)*> Fetch<'a> for ($($type_name, )*){
//             type Item = ($(&'a $type_name,)*);
//         }
//     };
// }
//
// // impl_fetch!(A0, a0, A1, a1);
//
// macro_rules! all_a {
//     ($macro_name: ident) => {
//         $macro_name!(A0, a0);
//         $macro_name!(A0, a0, A1, a1);
//         $macro_name!(A0, a0, A1, a1, A2, a2);
//         $macro_name!(A0, a0, A1, a1, A2, a2, A3, a3);
//         $macro_name!(A0, a0, A1, a1, A2, a2, A3, a3, A4, a4);
//         $macro_name!(A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5);
//         $macro_name!(A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6);
//         $macro_name!(A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6, A7, a7);
//         $macro_name!(A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6, A7, a7, A8, a8);
//     };
// }

// all_a!(impl_fetch);
