use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::convert::identity;

/// Entity is just id. You can assign components to Entity
#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub struct Entity(usize);

/// System says about set of components that will be tracked by ECS.
/// You can get a set of entities with these components using the SystemId
pub struct SystemId(usize);

pub struct ComponentArray<T> {
    components: Vec<T>,
    entity_to_index: HashMap<Entity, usize>,
}

impl<T> ComponentArray<T> {
    pub fn new() -> Self {
        ComponentArray {
            components: Vec::new(),
            entity_to_index: HashMap::new(),
        }
    }

    pub fn add_component(&mut self, e: Entity, comp: T) {
        assert!(!self.entity_to_index.contains_key(&e));
        self.entity_to_index.insert(e, self.components.len());
        self.components.push(comp);
    }

    pub fn remove_component(&mut self, e: Entity) {
        todo!();
    }

    pub fn get_component(&self, e: Entity) -> &T {
        let idx = self.get_idx(e);
        self.components.get(idx).unwrap()
    }

    pub fn get_component_mut(&mut self, e: Entity) -> &mut T {
        let idx = self.get_idx(e);
        self.components.get_mut(idx).unwrap()
    }

    ///

    fn get_idx(&self, e: Entity) -> usize {
        *self
            .entity_to_index
            .get(&e)
            .expect("This entity did not have this component")
    }
}


