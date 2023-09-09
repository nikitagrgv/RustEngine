use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::convert::identity;

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub struct Entity(i32);

pub struct ComponentArray<C> {
    components: Vec<C>,
    entity_to_index: HashMap<Entity, usize>,
}

impl<C> ComponentArray<C> {
    pub fn new() -> Self {
        ComponentArray {
            components: Vec::new(),
            entity_to_index: HashMap::new(),
        }
    }

    fn get_component(&self, e: Entity) -> Option<&C> {
        let idx = self.get_index(e)?;
        self.components.get(idx)
    }

    fn get_component_mut(&mut self, e: Entity) -> Option<&mut C> {
        let idx = self.get_index(e)?;
        self.components.get_mut(idx)
    }

    fn add_component(&mut self, e: Entity, component: C) {
        assert!(
            !self.entity_to_index.contains_key(&e),
            "Entity does not have this component"
        );
        self.entity_to_index.insert(e, self.components.len());
        self.components.push(component);
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

    pub fn register_component<C: 'static>(&mut self) {
        let type_id = TypeId::of::<C>();
        assert!(
            !self.type_to_components.contains_key(&type_id),
            "Already registered"
        );
        let component_array = ComponentArray::new();
        let component_array = Box::<ComponentArray<C>>::new(component_array);
        self.type_to_components.insert(type_id, component_array);
    }

    pub fn add_component<C: 'static>(&mut self, e: Entity, component: C) {
        let type_id = TypeId::of::<C>();
        let component_array = self
            .type_to_components
            .get_mut(&type_id)
            .expect("Component is not registered")
            .downcast_mut::<ComponentArray<C>>()
            .expect("Component array found but type not corresponded");
        component_array.add_component(e, component);
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

    pub fn register_component<C: 'static>(&mut self) {
        self.component_manager.register_component::<C>();
    }

    pub fn register_system<S>(&mut self) -> System {
        todo!()
    }
}

pub struct Signature {}

pub struct System {}
