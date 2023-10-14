use bevy::ptr::UnsafeCellDeref;
use std::any::{Any, TypeId};
use std::cell::{Ref, RefCell, RefMut, UnsafeCell};
use std::collections::{HashMap, HashSet};
use std::marker::PhantomData;

/// Entity is just id. You can assign components to Entity
#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
pub struct Entity(usize);

impl Entity {
    fn from_num(num: usize) -> Self {
        Self(num)
    }
    fn to_num(&self) -> usize {
        self.0
    }
}

pub trait Component: 'static {
    fn get_type_id() -> TypeId;
}

impl<T: 'static> Component for T {
    fn get_type_id() -> TypeId {
        TypeId::of::<T>()
    }
}

trait ComponentArray {
    fn push_none(&mut self);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub struct ComponentArrayT<T: Component> {
    components: UnsafeCell<Vec<Option<T>>>,
}

impl<T: Component> ComponentArrayT<T> {
    pub fn new() -> Self {
        Self {
            components: UnsafeCell::new(Vec::new()),
        }
    }
}

impl<T: Component> ComponentArray for ComponentArrayT<T> {
    fn push_none(&mut self) {
        self.components.get_mut().push(None);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

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
        let mut component_array = ComponentArrayT::<T>::new();
        for _ in 0..self.entities_count {
            component_array.push_none();
        }
        self.component_arrays
            .insert(type_id, Box::new(component_array));
    }

    pub fn add_component<T: Component>(&mut self, component: T, e: Entity) {
        unsafe {
            self.get_component_array::<T>()
                .expect("Component is not registered")
                .components
                .deref_mut()[e.to_num()] = Some(component).into();
        }
    }

    pub fn remove_component<T: Component>(&mut self, component: T, e: Entity) {
        unsafe {
            self.get_component_array::<T>()
                .expect("Component is not registered")
                .components
                .deref_mut()[e.to_num()] = None.into();
        }
    }

    pub fn get_component_array<T: Component>(&self) -> Option<&ComponentArrayT<T>> {
        let type_id = T::get_type_id();
        self.component_arrays
            .get(&type_id)?
            .as_any()
            .downcast_ref::<ComponentArrayT<T>>()
    }

    pub fn query<'w, T: Fetcherable>(&'w self) -> Query<'w, T> {
        Query::<'w, T>::new(self)
    }
}

pub struct Query<'w, T: Fetcherable> {
    // world: &'w mut World,
    pub fetch: T::Fetch<'w>,
}

impl<'w, T: Fetcherable> Query<'w, T> {
    pub fn new(world: &'w World) -> Self {
        let fetch = T::fetch_init(world);
        Self { fetch }
    }

    pub fn fetch_entity<'q>(&'q self, entity: Entity) -> FetchResult<T::Item<'q>> {
        T::fetch_entity(&self.fetch, entity)
    }

    pub fn fetch_entity_mut<'q>(&'q self, entity: Entity) -> FetchResult<T::ItemMut<'q>> {
        T::fetch_entity_mut(&self.fetch, entity)
    }

    pub fn iter<'q>(&'q self) -> QueryIter<'q, 'w, T> {
        QueryIter::<'q, 'w, T>::new(self)
    }
    pub fn iter_mut<'q>(&'q mut self) -> QueryIterMut<'q, 'w, T> {
        QueryIterMut::<'q, 'w, T>::new(self)
    }
}

pub struct QueryIter<'q, 'w: 'q, T: Fetcherable> {
    query: &'q Query<'w, T>,
    cur_entity: Entity,
}

impl<'q, 'w: 'q, T: Fetcherable> QueryIter<'q, 'w, T> {
    pub fn new(query: &'q Query<'w, T>) -> Self {
        Self {
            query,
            cur_entity: Entity::from_num(0),
        }
    }
}

impl<'q, 'w: 'q, T: Fetcherable> Iterator for QueryIter<'q, 'w, T> {
    type Item = T::Item<'q>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut cur_entity = self.cur_entity;
        let next = loop {
            match self.query.fetch_entity(cur_entity) {
                FetchResult::Some(c) => {
                    break Some(c);
                }
                FetchResult::None => cur_entity.0 += 1,
                FetchResult::End => {
                    break None;
                }
            }
        };
        self.cur_entity.0 += 1;
        next
    }
}

pub struct QueryIterMut<'q, 'w: 'q, T: Fetcherable> {
    query: &'q Query<'w, T>,
    cur_entity: Entity,
    marker: PhantomData<&'q mut Query<'w, T>>,
}

impl<'q, 'w: 'q, T: Fetcherable> QueryIterMut<'q, 'w, T> {
    pub fn new(query: &'q Query<'w, T>) -> Self {
        Self {
            query,
            cur_entity: Entity::from_num(0),
            marker: PhantomData,
        }
    }
}

impl<'q, 'w: 'q, T: Fetcherable> Iterator for QueryIterMut<'q, 'w, T> {
    type Item = T::ItemMut<'q>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut cur_entity = self.cur_entity;
        let next = loop {
            match self.query.fetch_entity_mut(cur_entity) {
                FetchResult::Some(c) => {
                    break Some(c);
                }
                FetchResult::None => cur_entity.0 += 1,
                FetchResult::End => {
                    break None;
                }
            }
        };
        self.cur_entity.0 += 1;
        next
    }
}

pub enum FetchResult<T> {
    Some(T),
    None,
    End,
}

pub trait Fetcherable {
    type Item<'w>;
    type ItemMut<'w>;
    type Fetch<'w>;

    fn fetch_init<'w>(world: &'w World) -> Self::Fetch<'w>;

    fn fetch_entity<'f, 'w: 'f>(
        fetch: &'f Self::Fetch<'w>,
        entity: Entity,
    ) -> FetchResult<Self::Item<'f>>;

    fn fetch_entity_mut<'f, 'w: 'f>(
        fetch: &'f Self::Fetch<'w>,
        entity: Entity,
    ) -> FetchResult<Self::ItemMut<'f>>;
}

impl<T: Component> Fetcherable for &T {
    type Item<'w> = &'w T;
    type ItemMut<'w> = &'w T;
    type Fetch<'w> = &'w ComponentArrayT<T>;

    fn fetch_init<'w>(world: &'w World) -> Self::Fetch<'w> {
        world.get_component_array::<T>().unwrap()
    }

    fn fetch_entity<'f, 'w: 'f>(
        fetch: &'f Self::Fetch<'w>,
        entity: Entity,
    ) -> FetchResult<Self::Item<'f>> {
        let comp_vec = unsafe { fetch.components.deref() };
        match comp_vec.get(entity.to_num()) {
            None => FetchResult::End,
            Some(comp) => match comp {
                None => FetchResult::None,
                Some(comp) => FetchResult::Some(comp),
            },
        }
    }

    fn fetch_entity_mut<'f, 'w: 'f>(
        fetch: &'f Self::Fetch<'w>,
        entity: Entity,
    ) -> FetchResult<Self::ItemMut<'f>> {
        let comp_vec = unsafe { fetch.components.deref_mut() };
        match comp_vec.get_mut(entity.to_num()) {
            None => FetchResult::End,
            Some(comp) => match comp {
                None => FetchResult::None,
                Some(comp) => FetchResult::Some(comp),
            },
        }
    }
}

impl<T: Component> Fetcherable for &mut T {
    type Item<'w> = &'w T;
    type ItemMut<'w> = &'w mut T;
    type Fetch<'w> = &'w ComponentArrayT<T>;

    fn fetch_init<'w>(world: &'w World) -> Self::Fetch<'w> {
        world.get_component_array::<T>().unwrap()
    }

    fn fetch_entity<'f, 'w: 'f>(
        fetch: &'f Self::Fetch<'w>,
        entity: Entity,
    ) -> FetchResult<Self::Item<'f>> {
        let comp_vec = unsafe { fetch.components.deref() };
        match comp_vec.get(entity.to_num()) {
            None => FetchResult::End,
            Some(comp) => match comp {
                None => FetchResult::None,
                Some(comp) => FetchResult::Some(comp),
            },
        }
    }

    fn fetch_entity_mut<'f, 'w: 'f>(
        fetch: &'f Self::Fetch<'w>,
        entity: Entity,
    ) -> FetchResult<Self::ItemMut<'f>> {
        let comp_vec = unsafe { fetch.components.deref_mut() };
        match comp_vec.get_mut(entity.to_num()) {
            None => FetchResult::End,
            Some(comp) => match comp {
                None => FetchResult::None,
                Some(comp) => FetchResult::Some(comp),
            },
        }
    }
}

macro_rules! impl_fetch_helper_1 {
    ($a: tt) => {
        FetchResult::None
    };
}

macro_rules! impl_fetch_helper_2 {
    ($a: tt) => {
        _
    };
}

macro_rules! impl_fetch {
    ($($type_name: ident, $var_name: ident, $num: tt),*) => {
        // impl<'a, $($type_name: 'static,)*> Fetch<'a> for ($($type_name, )*){
        //     type Item = ($(&'a $type_name,)*);
        // }
        impl<$($type_name : Fetcherable, )*> Fetcherable for ($($type_name, )*) {
            type Item<'w> = ($($type_name::Item<'w>, )*);
            type ItemMut<'w> = ($($type_name::ItemMut<'w>, )*);
            type Fetch<'w> = ($($type_name::Fetch<'w>, )*);

            fn fetch_init<'w>(world: &'w World) -> Self::Fetch<'w> {
                ($($type_name::fetch_init(world), )*)
            }

            fn fetch_entity<'f, 'w: 'f>(
                fetch: &'f Self::Fetch<'w>,
                entity: Entity,
            ) -> FetchResult<Self::Item<'f>> {
                match (
                    $($type_name::fetch_entity(&fetch.$num, entity), )*
                ) {
                    ($(FetchResult::Some($var_name),)*) => FetchResult::Some(($($var_name, )*)),
                    ($(impl_fetch_helper_1!($var_name), )*) => FetchResult::None,
                    ($(impl_fetch_helper_2!($var_name), )*) => FetchResult::End,
                }
            }

            fn fetch_entity_mut<'f, 'w: 'f>(
                fetch: &'f Self::Fetch<'w>,
                entity: Entity,
            ) -> FetchResult<Self::ItemMut<'f>> {
                match (
                    $($type_name::fetch_entity_mut(&fetch.$num, entity), )*
                ) {
                    ($(FetchResult::Some($var_name),)*) => FetchResult::Some(($($var_name, )*)),
                    ($(impl_fetch_helper_1!($var_name), )*) => FetchResult::None,
                    ($(impl_fetch_helper_2!($var_name), )*) => FetchResult::End,
                }
            }
        }
    };
}

macro_rules! all_tuples {
    ($m: ident) => {
        $m!(T0, t0, 0);
        $m!(T0, t0, 0, T1, t1, 1);
        $m!(T0, t0, 0, T1, t1, 1, T2, t2, 2);
        $m!(T0, t0, 0, T1, t1, 1, T2, t2, 2, T3, t3, 3);
        $m!(T0, t0, 0, T1, t1, 1, T2, t2, 2, T3, t3, 3, T4, t4, 4);
        $m!(T0, t0, 0, T1, t1, 1, T2, t2, 2, T3, t3, 3, T4, t4, 4, T5, t5, 5);
        $m!(T0, t0, 0, T1, t1, 1, T2, t2, 2, T3, t3, 3, T4, t4, 4, T5, t5, 5, T6, t6, 6);
        $m!(T0, t0, 0, T1, t1, 1, T2, t2, 2, T3, t3, 3, T4, t4, 4, T5, t5, 5, T6, t6, 6, T7, t7, 7);
    };
}

all_tuples!(impl_fetch);
