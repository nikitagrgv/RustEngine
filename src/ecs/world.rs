use crate::ecs::component::Component;
use crate::ecs::entity::Entity;
use crate::ecs::{ComponentArray, ComponentArrayCell, Fetcherable, Query};
use std::any::TypeId;
use std::collections::HashMap;

pub struct World {
    entities_count: usize,
    component_arrays: HashMap<TypeId, Box<dyn ComponentArray>>,
}

impl World {
    pub fn new() -> Self {
        Self {
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
        let mut component_array = ComponentArrayCell::<T>::new();
        for _ in 0..self.entities_count {
            component_array.push_none();
        }
        self.component_arrays
            .insert(type_id, Box::new(component_array));
    }

    pub fn add_component<T: Component>(&mut self, component: T, e: Entity) {
        unsafe {
            self.get_component_array_mut::<T>()
                .expect("Component is not registered")
                .components
                .get_mut()[e.to_num()] = Some(component).into();
        }
    }

    pub fn remove_component<T: Component>(&mut self, component: T, e: Entity) {
        unsafe {
            self.get_component_array_mut::<T>()
                .expect("Component is not registered")
                .components
                .get_mut()[e.to_num()] = None.into();
        }
    }

    pub fn get_component_array<T: Component>(&self) -> Option<&ComponentArrayCell<T>> {
        let type_id = T::get_type_id();
        self.component_arrays
            .get(&type_id)?
            .as_any()
            .downcast_ref::<ComponentArrayCell<T>>()
    }

    pub fn get_component_array_mut<T: Component>(&mut self) -> Option<&mut ComponentArrayCell<T>> {
        let type_id = T::get_type_id();
        self.component_arrays
            .get_mut(&type_id)?
            .as_any_mut()
            .downcast_mut::<ComponentArrayCell<T>>()
    }

    pub fn query<'w, T: Fetcherable>(&'w self) -> Query<'w, T> {
        Query::<'w, T>::new(self)
    }
}
