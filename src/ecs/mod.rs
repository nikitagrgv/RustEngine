use crate::utils::to_any::ToAny;
use std::any::TypeId;
use std::cell::{Ref, RefCell, RefMut};
use std::collections::{HashMap, HashSet};
use std::ops::{Deref, DerefMut};

/// Entity is just id. You can assign components to Entity
#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
pub struct Entity(usize);

/// System says about set of components that will be tracked by Ecs.
/// You can get a set of entities with these components using the SystemId
#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
pub struct SystemId(usize);

///////////////////////////////////////

pub struct ComponentArray<C> {
    components: Vec<RefCell<C>>,
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
        self.components.push(comp.into());
    }

    pub fn remove_component(&mut self, e: Entity) {
        todo!();
    }

    pub fn get_component(&self, e: Entity) -> Ref<C> {
        let idx = self.get_idx(e);
        self.components.get(idx).unwrap().borrow()
    }

    pub fn get_component_mut(&self, e: Entity) -> RefMut<C> {
        let idx = self.get_idx(e);
        self.components.get(idx).unwrap().borrow_mut()
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

    pub fn get_component<C: 'static>(&self, e: Entity) -> Ref<C> {
        self.get_component_array::<C>().get_component(e)
    }

    pub fn get_component_mut<C: 'static>(&self, e: Entity) -> RefMut<C> {
        self.get_component_array::<C>().get_component_mut(e)
    }

    pub fn on_entity_removed(&mut self, e: Entity) {
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

    pub fn add<C: 'static>(&mut self) -> &mut Self {
        let typeid = TypeId::of::<C>();
        self.components.insert(typeid);
        self
    }

    pub fn remove<C: 'static>(&mut self) {
        let typeid = TypeId::of::<C>();
        self.components.remove(&typeid);
    }

    pub fn has<C: 'static>(&self) -> bool {
        let typeid = TypeId::of::<C>();
        self.components.contains(&typeid)
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

    pub fn create_entity(&mut self) -> Entity {
        let e = self.get_free_entity();
        let sig = Signature::new();
        self.entity_to_signature.insert(e, sig);
        e
    }

    pub fn remove_entity(&mut self, e: Entity) {
        todo!();
    }

    pub fn get_signature(&mut self, e: Entity) -> &Signature {
        self.entity_to_signature.get(&e).expect("No such entity")
    }

    pub fn set_signature(&mut self, e: Entity, sig: Signature) {
        *self
            .entity_to_signature
            .get_mut(&e)
            .expect("No such entity") = sig;
    }

    pub fn add_to_signature<T: 'static>(&mut self, e: Entity) {
        self.entity_to_signature
            .get_mut(&e)
            .expect("No such entity")
            .add::<T>();
    }

    pub fn remove_from_signature<T: 'static>(&mut self, e: Entity) {
        self.entity_to_signature
            .get_mut(&e)
            .expect("No such entity")
            .remove::<T>();
    }

    pub fn has_in_signature<T: 'static>(&self, e: Entity) {
        self.entity_to_signature
            .get(&e)
            .expect("No such entity")
            .has::<T>();
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

pub struct System {
    signature: Signature,
    entities: Vec<Entity>,
}

impl System {
    pub fn new() -> Self {
        Self {
            signature: Signature::new(),
            entities: Vec::new(),
        }
    }

    pub fn with_sig(sig: Signature) -> Self {
        Self {
            signature: sig,
            entities: Vec::new(),
        }
    }

    pub fn get_signature(&mut self) -> &Signature {
        &self.signature
    }

    pub fn set_signature(&mut self, sig: Signature) {
        self.signature = sig;
    }

    pub fn has_component<C: 'static>(&self) -> bool {
        self.signature.has::<C>()
    }

    pub fn get_entities(&self) -> &Vec<Entity> {
        &self.entities
    }
}

pub struct SystemManager {
    id_to_system: HashMap<SystemId, System>,
}

impl SystemManager {
    pub fn new() -> Self {
        Self {
            id_to_system: HashMap::new(),
        }
    }

    pub fn create_system_with_signature(&mut self, sig: Signature) -> SystemId {
        let id = self.get_free_system_id();
        let sys = System::with_sig(sig);
        self.id_to_system.insert(id, sys);
        id
    }

    pub fn create_system(&mut self) -> SystemId {
        let id = self.get_free_system_id();
        let sys = System::new();
        self.id_to_system.insert(id, sys);
        id
    }

    pub fn remove_system(&mut self, s: SystemId) {
        todo!();
    }

    pub fn get_signature(&mut self, s: SystemId) -> &Signature {
        &self.id_to_system.get(&s).expect("No such system").signature
    }

    pub fn set_signature(&mut self, s: SystemId, sig: Signature) {
        self.id_to_system
            .get_mut(&s)
            .expect("No such system")
            .set_signature(sig);
    }

    pub fn get_entities(&self, s: SystemId) -> &Vec<Entity> {
        self.id_to_system
            .get(&s)
            .expect("No such system")
            .get_entities()
    }

    fn get_free_system_id(&self) -> SystemId {
        // TODO: find better way
        let mut s = SystemId(0);
        loop {
            if !self.id_to_system.contains_key(&s) {
                break s;
            }
            s.0 = s.0 + 1;
        }
    }
}

pub struct Ecs {
    entity_manager: EntityManager,
    component_manager: ComponentManager,
    system_manager: SystemManager,
}

impl Ecs {
    pub fn new() -> Self {
        Self {
            entity_manager: EntityManager::new(),
            component_manager: ComponentManager::new(),
            system_manager: SystemManager::new(),
        }
    }

    ////////////// Entity

    pub fn create_entity(&mut self) -> Entity {
        self.entity_manager.create_entity()
    }

    pub fn remove_entity(&mut self, e: Entity) {
        self.entity_manager.remove_entity(e);
    }

    pub fn get_entity_signature(&mut self, e: Entity) -> &Signature {
        self.entity_manager.get_signature(e)
    }

    ////////////// Component

    pub fn register_component<C: 'static>(&mut self) -> &mut Self {
        self.component_manager.register_component::<C>();
        self
    }

    pub fn add_component<C: 'static>(&mut self, e: Entity, comp: C) {
        self.component_manager.add_component::<C>(e, comp);
    }

    pub fn remove_component<C: 'static>(&mut self, e: Entity) {
        self.component_manager.remove_component::<C>(e);
    }

    pub fn get_component<C: 'static>(&self, e: Entity) -> Ref<C> {
        self.component_manager.get_component(e)
    }

    pub fn get_component_mut<C: 'static>(&self, e: Entity) -> RefMut<C> {
        self.component_manager.get_component_mut(e)
    }

    ////////////// System

    pub fn create_system_with_signature(&mut self, sig: Signature) -> SystemId {
        self.system_manager.create_system_with_signature(sig)
    }

    pub fn create_system(&mut self) -> SystemId {
        self.system_manager.create_system()
    }

    pub fn remove_system(&mut self, s: SystemId) {
        self.system_manager.remove_system(s);
    }

    pub fn get_system_signature(&mut self, s: SystemId) -> &Signature {
        self.system_manager.get_signature(s)
    }

    pub fn set_system_signature(&mut self, s: SystemId, sig: Signature) {
        self.system_manager.set_signature(s, sig);
    }

    pub fn get_system_entities(&self, s: SystemId) -> &Vec<Entity> {
        self.system_manager.get_entities(s)
    }
}
