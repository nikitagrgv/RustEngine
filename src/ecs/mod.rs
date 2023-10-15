mod entity;
mod component;
mod world;

use bevy::ptr::UnsafeCellDeref;
use std::any::{Any};
use std::cell::{Cell, UnsafeCell};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;

pub use entity::*;
pub use component::*;
pub use world::*;


trait ComponentArray {
    fn push_none(&mut self);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

type BorrowFlag = isize;
const UNUSED: BorrowFlag = 0;
fn is_writing(x: BorrowFlag) -> bool {
    x < UNUSED
}
fn is_reading(x: BorrowFlag) -> bool {
    x > UNUSED
}

struct ComponentArrayBorrowRef<'b> {
    borrow: &'b Cell<BorrowFlag>,
}

impl<'b> ComponentArrayBorrowRef<'b> {
    #[inline]
    fn new(borrow: &'b Cell<BorrowFlag>) -> Option<ComponentArrayBorrowRef<'b>> {
        let b = borrow.get().wrapping_add(1);
        if !is_reading(b) {
            None
        } else {
            borrow.set(b);
            Some(ComponentArrayBorrowRef { borrow })
        }
    }
}

impl Drop for ComponentArrayBorrowRef<'_> {
    #[inline]
    fn drop(&mut self) {
        let borrow = self.borrow.get();
        debug_assert!(is_reading(borrow));
        self.borrow.set(borrow - 1);
    }
}

impl Clone for ComponentArrayBorrowRef<'_> {
    #[inline]
    fn clone(&self) -> Self {
        // Since this Ref exists, we know the borrow flag
        // is a reading borrow.
        let borrow = self.borrow.get();
        debug_assert!(is_reading(borrow));
        // Prevent the borrow counter from overflowing into
        // a writing borrow.
        assert!(borrow != isize::MAX);
        self.borrow.set(borrow + 1);
        ComponentArrayBorrowRef {
            borrow: self.borrow,
        }
    }
}

struct ComponentArrayBorrowRefMut<'b> {
    borrow: &'b Cell<BorrowFlag>,
}

impl Drop for ComponentArrayBorrowRefMut<'_> {
    #[inline]
    fn drop(&mut self) {
        let borrow = self.borrow.get();
        debug_assert!(is_writing(borrow));
        self.borrow.set(borrow + 1);
    }
}

impl<'b> ComponentArrayBorrowRefMut<'b> {
    #[inline]
    fn new(borrow: &'b Cell<BorrowFlag>) -> Option<ComponentArrayBorrowRefMut<'b>> {
        match borrow.get() {
            UNUSED => {
                borrow.set(UNUSED - 1);
                Some(ComponentArrayBorrowRefMut { borrow })
            }
            _ => None,
        }
    }

    #[inline]
    fn clone(&self) -> ComponentArrayBorrowRefMut<'b> {
        let borrow = self.borrow.get();
        debug_assert!(is_writing(borrow));
        assert!(borrow != isize::MIN);
        self.borrow.set(borrow - 1);
        ComponentArrayBorrowRefMut {
            borrow: self.borrow,
        }
    }
}

pub struct ComponentArrayRef<'b, T: Component + 'b> {
    value: NonNull<Vec<Option<T>>>,
    borrow: ComponentArrayBorrowRef<'b>,
}

impl<T: Component> Deref for ComponentArrayRef<'_, T> {
    type Target = Vec<Option<T>>;

    fn deref(&self) -> &Vec<Option<T>> {
        unsafe { self.value.as_ref() }
    }
}

pub struct ComponentArrayRefMut<'b, T: Component + 'b> {
    value: NonNull<Vec<Option<T>>>,
    borrow: ComponentArrayBorrowRefMut<'b>,
    marker: PhantomData<&'b mut T>,
}

impl<'b, T: Component + 'b> ComponentArrayRefMut<'b, T> {
    unsafe fn deref_mut_unsafe(&self) -> &mut Vec<Option<T>> {
        unsafe { NonNull::new_unchecked(self.value.as_ptr()).as_mut() }
    }
}

impl<T: Component> Deref for ComponentArrayRefMut<'_, T> {
    type Target = Vec<Option<T>>;

    fn deref(&self) -> &Vec<Option<T>> {
        unsafe { self.value.as_ref() }
    }
}

impl<T: Component> DerefMut for ComponentArrayRefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Vec<Option<T>> {
        unsafe { self.value.as_mut() }
    }
}

pub struct ComponentArrayCell<T: Component> {
    borrow: Cell<BorrowFlag>,
    components: UnsafeCell<Vec<Option<T>>>,
}

impl<T: Component> ComponentArrayCell<T> {
    pub fn new() -> Self {
        Self {
            borrow: Cell::new(UNUSED),
            components: UnsafeCell::new(Vec::new()),
        }
    }

    pub fn borrow(&self) -> ComponentArrayRef<'_, T> {
        self.try_borrow().expect("already mutably borrowed")
    }

    pub fn borrow_mut(&self) -> ComponentArrayRefMut<'_, T> {
        self.try_borrow_mut().expect("already borrowed")
    }

    pub fn try_borrow(&self) -> Option<ComponentArrayRef<'_, T>> {
        match ComponentArrayBorrowRef::new(&self.borrow) {
            Some(b) => {
                let value = unsafe { NonNull::new_unchecked(self.components.get()) };
                Some(ComponentArrayRef { value, borrow: b })
            }
            None => None,
        }
    }

    pub fn try_borrow_mut(&self) -> Option<ComponentArrayRefMut<'_, T>> {
        match ComponentArrayBorrowRefMut::new(&self.borrow) {
            Some(b) => {
                let value = unsafe { NonNull::new_unchecked(self.components.get()) };
                Some(ComponentArrayRefMut {
                    value,
                    borrow: b,
                    marker: PhantomData,
                })
            }
            None => None,
        }
    }

    pub fn get_mut(&mut self) -> &mut Vec<Option<T>> {
        self.components.get_mut()
    }
}

impl<T: Component> ComponentArray for ComponentArrayCell<T> {
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
    type Fetch<'w> = ComponentArrayRef<'w, T>;

    fn fetch_init<'w>(world: &'w World) -> Self::Fetch<'w> {
        world.get_component_array::<T>().unwrap().borrow()
    }

    fn fetch_entity<'f, 'w: 'f>(
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

    fn fetch_entity_mut<'f, 'w: 'f>(
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

impl<T: Component> Fetcherable for &mut T {
    type Item<'w> = &'w T;
    type ItemMut<'w> = &'w mut T;
    type Fetch<'w> = ComponentArrayRefMut<'w, T>;

    fn fetch_init<'w>(world: &'w World) -> Self::Fetch<'w> {
        world.get_component_array::<T>().unwrap().borrow_mut()
    }

    fn fetch_entity<'f, 'w: 'f>(
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

    fn fetch_entity_mut<'f, 'w: 'f>(
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
