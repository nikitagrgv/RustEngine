use crate::world::{CARef, CARefMut, Component, Entity, World};
use std::marker::PhantomData;

pub struct Query<'w, T: Fetcherable> {
    pub fetch: T::Fetch<'w>,
}

impl<'w, T: Fetcherable> Query<'w, T> {
    pub fn new(world: &'w World) -> Self {
        let fetch = T::fetch_init(world);
        Self { fetch }
    }

    pub unsafe fn fetch_entity<'q>(&'q self, entity: Entity) -> FetchResult<T::Item<'q>> {
        T::fetch_entity(&self.fetch, entity)
    }

    pub unsafe fn fetch_entity_mut<'q>(&'q self, entity: Entity) -> FetchResult<T::ItemMut<'q>> {
        T::fetch_entity_mut(&self.fetch, entity)
    }

    pub fn iter<'q>(&'q self) -> QueryIter<'q, 'w, T> {
        QueryIter::<'q, 'w, T>::new(self)
    }
    pub fn iter_mut<'q>(&'q mut self) -> QueryIterMut<'q, 'w, T> {
        QueryIterMut::<'q, 'w, T>::new(self)
    }
}

/// QueryIter
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

pub struct QueryIterItem<'q, T: Fetcherable> {
    pub ent: Entity,
    pub comp: T::Item<'q>,
}

impl<'q, 'w: 'q, T: Fetcherable> Iterator for QueryIter<'q, 'w, T> {
    type Item = QueryIterItem<'q, T>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match unsafe { self.query.fetch_entity(self.cur_entity) } {
                FetchResult::Some(comp) => {
                    let ent = self.cur_entity;
                    self.cur_entity.0 += 1;
                    break Some(QueryIterItem { ent, comp });
                }
                FetchResult::None => self.cur_entity.0 += 1,
                FetchResult::End => {
                    break None;
                }
            }
        }
    }
}

/// QueryIterMut
pub struct QueryIterMut<'q, 'w: 'q, T: Fetcherable> {
    query: &'q Query<'w, T>,
    // TODO: shit?
    // Needed for skipping iterator - we need to know what entity to skip
    last_given_entity: Option<Entity>,
    cur_entity: Entity,
    marker: PhantomData<&'q mut Query<'w, T>>,
}

impl<'q, 'w: 'q, T: Fetcherable> QueryIterMut<'q, 'w, T> {
    pub fn new(query: &'q Query<'w, T>) -> Self {
        Self {
            query,
            last_given_entity: None,
            cur_entity: Entity::from_num(0),
            marker: PhantomData,
        }
    }

    pub fn iter_skipping_current(&self) -> QueryIterSkip<'q, 'w, T> {
        QueryIterSkip {
            query: self.query,
            cur_entity: Entity::from_num(0),
            skip_entity: self
                .last_given_entity
                .expect("No current item. Don't call this method before or after iterating"),
        }
    }
}

pub struct QueryIterMutItem<'q, T: Fetcherable> {
    pub ent: Entity,
    pub comp: T::ItemMut<'q>,
}

impl<'q, 'w: 'q, T: Fetcherable> Iterator for QueryIterMut<'q, 'w, T> {
    type Item = QueryIterMutItem<'q, T>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match unsafe { self.query.fetch_entity_mut(self.cur_entity) } {
                FetchResult::Some(comp) => {
                    let ent = self.cur_entity;
                    self.last_given_entity = Some(ent);
                    self.cur_entity.0 += 1;
                    break Some(QueryIterMutItem { ent, comp });
                }
                FetchResult::None => self.cur_entity.0 += 1,
                FetchResult::End => {
                    self.last_given_entity = None;
                    break None;
                }
            }
        }
    }
}

/// QueryIterSkip
pub struct QueryIterSkip<'q, 'w: 'q, T: Fetcherable> {
    query: &'q Query<'w, T>,
    cur_entity: Entity,
    skip_entity: Entity,
}

impl<'q, 'w: 'q, T: Fetcherable> QueryIterSkip<'q, 'w, T> {
    pub fn new(query: &'q Query<'w, T>, skip_entity: Entity) -> Self {
        Self {
            query,
            cur_entity: Entity::from_num(0),
            skip_entity,
        }
    }
}

impl<'q, 'w: 'q, T: Fetcherable> Iterator for QueryIterSkip<'q, 'w, T> {
    type Item = QueryIterItem<'q, T>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if (self.cur_entity == self.skip_entity) {
                self.cur_entity.0 += 1;
            }

            match unsafe { self.query.fetch_entity(self.cur_entity) } {
                FetchResult::Some(comp) => {
                    let ent = self.cur_entity;
                    self.cur_entity.0 += 1;
                    break Some(QueryIterItem { ent, comp });
                }
                FetchResult::None => self.cur_entity.0 += 1,
                FetchResult::End => {
                    break None;
                }
            }
        }
    }
}

/// FetchResult
pub enum FetchResult<T> {
    Some(T),
    None,
    End,
}

/// Fetcherable
pub trait Fetcherable {
    type Item<'w>;
    type ItemMut<'w>;
    type Fetch<'w>;

    fn fetch_init<'w>(world: &'w World) -> Self::Fetch<'w>;

    unsafe fn fetch_entity<'f, 'w: 'f>(
        fetch: &'f Self::Fetch<'w>,
        entity: Entity,
    ) -> FetchResult<Self::Item<'f>>;

    unsafe fn fetch_entity_mut<'f, 'w: 'f>(
        fetch: &'f Self::Fetch<'w>,
        entity: Entity,
    ) -> FetchResult<Self::ItemMut<'f>>;
}

/// Fetcherable for &T
impl<T: Component> Fetcherable for &T {
    type Item<'w> = &'w T;
    type ItemMut<'w> = &'w T;
    type Fetch<'w> = CARef<'w, T>;

    fn fetch_init<'w>(world: &'w World) -> Self::Fetch<'w> {
        world.get_component_array::<T>().unwrap().borrow()
    }

    unsafe fn fetch_entity<'f, 'w: 'f>(
        fetch: &'f Self::Fetch<'w>,
        entity: Entity,
    ) -> FetchResult<Self::Item<'f>> {
        match fetch.get(entity.to_num()) {
            None => FetchResult::End,
            Some(comp) => match comp {
                None => FetchResult::None,
                Some(comp) => FetchResult::Some(comp),
            },
        }
    }

    unsafe fn fetch_entity_mut<'f, 'w: 'f>(
        fetch: &'f Self::Fetch<'w>,
        entity: Entity,
    ) -> FetchResult<Self::ItemMut<'f>> {
        match fetch.get(entity.to_num()) {
            None => FetchResult::End,
            Some(comp) => match comp {
                None => FetchResult::None,
                Some(comp) => FetchResult::Some(comp),
            },
        }
    }
}

/// Fetcherable for &mut T
impl<T: Component> Fetcherable for &mut T {
    type Item<'w> = &'w T;
    type ItemMut<'w> = &'w mut T;
    type Fetch<'w> = CARefMut<'w, T>;

    fn fetch_init<'w>(world: &'w World) -> Self::Fetch<'w> {
        world
            .get_component_array::<T>()
            .expect("No such component")
            .borrow_mut()
    }

    unsafe fn fetch_entity<'f, 'w: 'f>(
        fetch: &'f Self::Fetch<'w>,
        entity: Entity,
    ) -> FetchResult<Self::Item<'f>> {
        match fetch.get(entity.to_num()) {
            None => FetchResult::End,
            Some(comp) => match comp {
                None => FetchResult::None,
                Some(comp) => FetchResult::Some(comp),
            },
        }
    }

    unsafe fn fetch_entity_mut<'f, 'w: 'f>(
        fetch: &'f Self::Fetch<'w>,
        entity: Entity,
    ) -> FetchResult<Self::ItemMut<'f>> {
        unsafe {
            match fetch.deref_mut_unsafe().get_mut(entity.to_num()) {
                None => FetchResult::End,
                Some(comp) => match comp {
                    None => FetchResult::None,
                    Some(comp) => FetchResult::Some(comp),
                },
            }
        }
    }
}

/// Fetcherable for tuple
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

macro_rules! impl_fetch_for_tuple {
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

            unsafe fn fetch_entity<'f, 'w: 'f>(
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

            unsafe fn fetch_entity_mut<'f, 'w: 'f>(
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

all_tuples!(impl_fetch_for_tuple);
