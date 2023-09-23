use crate::utils::to_any::ToAny;
use std::any::{Any, TypeId};
use std::cell::{Ref, RefCell, RefMut};
use std::collections::{HashMap, HashSet};
use std::ops::{Deref, DerefMut};

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
        for mut component_array in &mut self.component_arrays {
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

    pub fn add_component<T: 'static>(&mut self, e: Entity, component: T) {
        self.get_component_array_mut::<T>()
            .expect("Component is not registered")
            .components[e.0] = Some(component);
    }

    pub fn remove_component<T: 'static>(&mut self, e: Entity, component: T) {
        self.get_component_array_mut::<T>()
            .expect("Component is not registered")
            .components[e.0] = None;
    }

    pub fn get_component_array<T: 'static>(&self) -> Option<Ref<ComponentArrayTemplate<T>>> {
        // TODO: wtf is this shittttt?
        for c in self.component_arrays.iter() {
            let is_such = c
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
}
