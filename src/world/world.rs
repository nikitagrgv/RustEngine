use crate::world::component::Component;
use crate::world::entity::Entity;
use crate::world::{CACell, ComponentArray, Fetcherable, Query};
use std::any::TypeId;
use std::collections::HashMap;

// struct HierarchyNode {
//     // TODO: use NonNull?
//     parent: Option<Entity>,
//     children: Vec<Entity>,
// }
//
// impl HierarchyNode {
//     fn new() -> Self {
//         Self {
//             parent: None,
//             children: Vec::new(),
//         }
//     }
//
//     fn is_child(&self, e: Entity) -> bool {
//         self.children.contains(&e)
//     }
//
//     fn is_parent(&self, e: Entity) -> bool {
//         match self.parent {
//             None => false,
//             Some(parent) => parent == e,
//         }
//     }
//
//     fn has_parent(&self) -> bool {
//         self.parent.is_some()
//     }
// }

pub struct World {
    // // idx - entity id
    // entities: Vec<HierarchyNode>,
    entities_count: usize,
    component_arrays: HashMap<TypeId, Box<dyn ComponentArray>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            // entities: Vec::new(),
            entities_count: 0,
            component_arrays: HashMap::new(),
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        let entity = Entity::from_num(self.entities_count);
        self.entities_count += 1;
        for component_array in self.component_arrays.values_mut() {
            component_array.push_none();
        }
        entity
    }

    pub fn register_component<T: Component>(&mut self) {
        let type_id = T::get_type_id();
        assert!(
            self.component_arrays.get(&type_id).is_none(),
            "Already registered"
        );
        let mut component_array = CACell::<T>::new();
        for _ in 0..self.entities_count {
            component_array.push_none();
        }
        self.component_arrays
            .insert(type_id, Box::new(component_array));
    }

    pub fn set_component<T: Component>(&mut self, component: T, entity: Entity) {
        unsafe {
            self.get_component_array_mut::<T>()
                .expect("Component is not registered")
                .set_component(component, entity)
        }
    }

    pub fn remove_component<T: Component>(&mut self, entity: Entity) {
        unsafe {
            self.get_component_array_mut::<T>()
                .expect("Component is not registered")
                .remove_component(entity);
        }
    }

    pub fn get_component_array<T: Component>(&self) -> Option<&CACell<T>> {
        let type_id = T::get_type_id();
        self.component_arrays
            .get(&type_id)?
            .as_any()
            .downcast_ref::<CACell<T>>()
    }

    pub fn get_component_array_mut<T: Component>(&mut self) -> Option<&mut CACell<T>> {
        let type_id = T::get_type_id();
        self.component_arrays
            .get_mut(&type_id)?
            .as_any_mut()
            .downcast_mut::<CACell<T>>()
    }

    pub fn query<'w, T: Fetcherable>(&'w self) -> Query<'w, T> {
        Query::<'w, T>::new(self)
    }
}
