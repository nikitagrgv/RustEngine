use crate::utils::to_any::ToAny;
use std::any::TypeId;
use std::collections::{HashMap, HashSet};
use std::ops::{Deref, DerefMut};

/// Entity is just id. You can assign components to Entity
#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub struct Entity(usize);

/// System says about set of components that will be tracked by ECS.
/// You can get a set of entities with these components using the SystemId
pub struct SystemId(usize);

///////////////////////////////////////

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

    fn get_idx(&self, e: Entity) -> usize {
        *self
            .entity_to_index
            .get(&e)
            .expect("This entity did not have this component")
    }
}

trait ComponentArrayBase: ToAny {
    fn on_entity_removed(&mut self, e: Entity);
}

impl<C: 'static> ComponentArrayBase for ComponentArray<C> {
    fn on_entity_removed(&mut self, e: Entity) {
        todo!()
    }
}

///////////////////////////////////////

pub struct ComponentManager {
    // TODO: not use TypeId?
    type_to_component_array: HashMap<TypeId, Box<dyn ComponentArrayBase>>,
}

impl ComponentManager {
    pub fn new() -> Self {
        Self {
            type_to_component_array: HashMap::new(),
        }
    }

    pub fn register_component<C: 'static>(&mut self) {
        let typeid = TypeId::of::<C>();
        assert!(
            !self.type_to_component_array.contains_key(&typeid),
            "Component is already registered"
        );
        let ca = ComponentArray::<C>::new();
        self.type_to_component_array.insert(typeid, Box::new(ca));
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

    pub fn remove_entity(&mut self, e: Entity) {
        self.type_to_component_array
            .values_mut()
            .for_each(|ca| ca.on_entity_removed(e));
    }

    fn get_component_array<C: 'static>(&self) -> &ComponentArray<C> {
        let typeid = TypeId::of::<C>();
        self.type_to_component_array
            .get(&typeid)
            .expect("Component is not registered")
            .deref()
            .as_any()
            .downcast_ref::<ComponentArray<C>>()
            .expect("Component found but types are not matched")
    }

    fn get_component_array_mut<C: 'static>(&mut self) -> &mut ComponentArray<C> {
        let typeid = TypeId::of::<C>();
        self.type_to_component_array
            .get_mut(&typeid)
            .expect("Component is not registered")
            .deref_mut()
            .as_any_mut()
            .downcast_mut::<ComponentArray<C>>()
            .expect("Component found but types are not matched")
    }
}

///////////////////////////////////////

#[derive(Eq, PartialEq, Clone)]
pub struct Signature {
    // TODO: use bitset and not use TypeId
    components: HashSet<TypeId>,
}

impl Signature {
    pub fn new() -> Signature {
        Self {
            components: HashSet::new(),
        }
    }

    pub fn add_component<T: 'static>(&mut self) {
        let typeid = TypeId::of::<T>();
        self.components.insert(typeid);
    }

    pub fn remove_component<T: 'static>(&mut self) {
        let typeid = TypeId::of::<T>();
        self.components.remove(&typeid);
    }

    pub fn has_component<T: 'static>(&self) {
        let typeid = TypeId::of::<T>();
        self.components.contains(&typeid);
    }
}

pub struct EntityManager {
    entity_to_signature: HashMap<Entity, Signature>,
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            entity_to_signature: HashMap::new(),
        }
    }

    pub fn create_entity_with_sig(&mut self, sig: Signature) -> Entity {
        let e = self.get_free_entity();
        self.entity_to_signature.insert(e, sig);
        e
    }

    pub fn create_entity(&mut self) -> Entity {
        let e = self.get_free_entity();
        let sig = Signature::new();
        self.entity_to_signature.insert(e, sig);
        e
    }

    pub fn remove_entity(&mut self, e: Entity) {
        todo!();
    }

    pub fn get_signature_mut(&mut self, e: Entity) -> &mut Signature {
        self.entity_to_signature
            .get_mut(&e)
            .expect("No such entity")
    }

    pub fn get_signature_ref(&mut self, e: Entity) -> &Signature {
        self.entity_to_signature.get(&e).expect("No such entity")
    }

    pub fn set_signature(&mut self, e: Entity, sig: Signature) {
        *self
            .entity_to_signature
            .get_mut(&e)
            .expect("No such entity") = sig;
    }

    fn get_free_entity(&self) -> Entity {
        // TODO: find better way
        let mut e = Entity(0);
        loop {
            if !self.entity_to_signature.contains_key(&e) {
                break e;
            }
            e.0 = e.0 + 1;
        }
    }
}
