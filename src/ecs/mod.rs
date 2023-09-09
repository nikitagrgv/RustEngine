use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::convert::identity;

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub struct Entity(i32);

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

    fn get_data(&self, e: Entity) -> Option<&T> {
        let idx = self.get_index(e)?;
        self.components.get(idx)
    }

    fn get_data_mut(&mut self, e: Entity) -> Option<&mut T> {
        let idx = self.get_index(e)?;
        self.components.get_mut(idx)
    }

    fn add_data(&mut self, e: Entity, data: T) {
        assert!(
            !self.entity_to_index.contains_key(&e),
            "Entity does not have this component"
        );
        self.entity_to_index.insert(e, self.components.len());
        self.components.push(data);
    }

    fn remove(&mut self, e: Entity) {
        todo!();
    }

    fn get_index(&self, e: Entity) -> Option<usize> {
        self.entity_to_index.get(&e).map(|idx| *idx)
    }
}

pub struct ComponentManager {
    type_to_components: HashMap<TypeId, Box<dyn Any>>,
}

impl ComponentManager {
    pub fn new() -> Self {
        ComponentManager {
            type_to_components: HashMap::new(),
        }
    }

    pub fn register_component<T: 'static>(&mut self) {
        let type_id = TypeId::of::<T>();
        assert!(
            !self.type_to_components.contains_key(&type_id),
            "Already registered"
        );
        let component_array = ComponentArray::new();
        let component_array = Box::<ComponentArray<T>>::new(component_array);
        self.type_to_components.insert(type_id, component_array);
    }

    pub fn add_data<T: 'static>(&mut self, e: Entity, data: T) {
        let type_id = TypeId::of::<T>();
        let component_array = self
            .type_to_components
            .get_mut(&type_id)
            .expect("Component is not registered")
            .downcast_mut::<ComponentArray<T>>()
            .expect("Component array found but type not corresponded");
        component_array.add_data(e, data);
    }
}

struct EntityManager {
    entities: Vec<Entity>,
}

impl EntityManager {
    pub fn new() -> Self {
        EntityManager {
            entities: Vec::new(),
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        let e = self.get_free_entity();
        self.entities.push(e);
        e
    }

    fn get_free_entity(&self) -> Entity {
        // TODO: optimize
        let mut e = Entity(0);
        loop {
            if (!self.entities.contains(&e)) {
                break e;
            }
            e.0 = e.0 + 1;
        }
    }
}

pub struct Ecs {
    component_manager: ComponentManager,
    entity_manager: EntityManager,
}

impl Ecs {
    pub fn new() -> Self {
        Ecs {
            component_manager: ComponentManager::new(),
            entity_manager: EntityManager::new(),
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        self.entity_manager.create_entity()
    }

    pub fn register_component<T: 'static>(&mut self) {
        self.component_manager.register_component::<T>();
    }

    pub fn register_system<T>(&mut self) -> System {
        todo!()
    }
}

pub struct Signature {}

pub struct System {}
