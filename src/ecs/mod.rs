use crate::utils;
use crate::utils::to_any::ToAny;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::convert::identity;

/// Entity is just id. You can assign components to Entity
#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub struct Entity(usize);

/// System says about set of components that will be tracked by ECS.
/// You can get a set of entities with these components using the SystemId
pub struct SystemId(usize);

pub struct ComponentArray<C> {
    components: Vec<C>,
    entity_to_index: HashMap<Entity, usize>,
    // TODO: index_to_entity?
}

impl<C: 'static> ComponentArray<C> {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
            entity_to_index: HashMap::new(),
        }
    }

    pub fn add_component(&mut self, e: Entity, comp: C) {
        assert!(!self.entity_to_index.contains_key(&e));
        self.entity_to_index.insert(e, self.components.len());
        self.components.push(comp);
    }

    pub fn remove_component(&mut self, e: Entity) {
        todo!();
    }

    pub fn get_component(&self, e: Entity) -> &C {
        let idx = self.get_idx(e);
        self.components.get(idx).unwrap()
    }

    pub fn get_component_mut(&mut self, e: Entity) -> &mut C {
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

trait ComponentArrayBase: ToAny {}
impl<C: 'static> ComponentArrayBase for ComponentArray<C> {}

struct ComponentManager {
    type_to_component: HashMap<TypeId, Box<dyn ComponentArrayBase>>,
}

impl ComponentManager {
    pub fn new() -> Self {
        Self {
            type_to_component: HashMap::new(),
        }
    }

    pub fn register_component<C: 'static>(&mut self) {
        let typeid = TypeId::of::<C>();
        assert!(
            !self.type_to_component.contains_key(&typeid),
            "Component is already registered"
        );
        let ca = ComponentArray::<C>::new();
        self.type_to_component.insert(typeid, Box::new(ca));
    }

    pub fn add_component<C: 'static>(&mut self, e: Entity, comp: C) {
        self.get_component_array_mut::<C>().add_component(e, comp);
    }

    pub fn remove_component<C: 'static>(&mut self, e: Entity) {
        self.get_component_array_mut::<C>().remove_component(e);
    }

    pub fn get_component<C: 'static>(&self, e: Entity) -> &C {
        self.get_component_array::<C>().get_component(e)
    }

    pub fn get_component_mut<C: 'static>(&mut self, e: Entity) -> &mut C {
        self.get_component_array_mut::<C>().get_component_mut(e)
    }

    fn get_component_array<C: 'static>(&self) -> &ComponentArray<C> {
        let typeid = TypeId::of::<C>();
        self.type_to_component
            .get(&typeid)
            .expect("Component is not registered")
            .as_any()
            .downcast_ref::<ComponentArray<C>>()
            .expect("Component found but types are not matched")
    }

    fn get_component_array_mut<C: 'static>(&mut self) -> &mut ComponentArray<C> {
        let typeid = TypeId::of::<C>();
        self.type_to_component
            .get_mut(&typeid)
            .expect("Component is not registered")
            .as_any_mut()
            .downcast_mut::<ComponentArray<C>>()
            .expect("Component found but types are not matched")
    }
}
