use bevy::ecs::query::Has;
use bevy::render::render_resource::encase::internal::{BufferMut, WriteInto, Writer};
use std::any::{Any, TypeId};
use std::cell::{Ref, RefCell, RefMut, UnsafeCell};
use std::collections::{HashMap, HashSet};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut, Index};

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
    components: Vec<UnsafeCell<Option<T>>>,
}

impl<T: Component> ComponentArrayT<T> {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }
}

impl<T: Component> ComponentArray for ComponentArrayT<T> {
    fn push_none(&mut self) {
        self.components.push(None.into());
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
        self.get_component_array_mut::<T>()
            .expect("Component is not registered")
            .components[e.to_num()] = Some(component).into();
    }

    pub fn remove_component<T: Component>(&mut self, component: T, e: Entity) {
        self.get_component_array_mut::<T>()
            .expect("Component is not registered")
            .components[e.to_num()] = None.into();
    }

    pub fn get_component_array<T: Component>(&self) -> Option<&ComponentArrayT<T>> {
        let type_id = T::get_type_id();
        self.component_arrays
            .get(&type_id)?
            .as_any()
            .downcast_ref::<ComponentArrayT<T>>()
    }

    pub fn get_component_array_mut<T: Component>(&mut self) -> Option<&mut ComponentArrayT<T>> {
        let type_id = T::get_type_id();
        self.component_arrays
            .get_mut(&type_id)?
            .as_any_mut()
            .downcast_mut::<ComponentArrayT<T>>()
    }

    pub fn query_mut<T: Fetcherable>(&mut self) -> Query<T> {
        Query::<T>::new(self)
    }

    // pub fn iter2<T0: 'static, T1: 'static>(&self) -> ComponentIterator2<T0, T1> {
    //     ComponentIterator2 {
    //         cur_ent: Entity(0),
    //         comp_arr_ref_0: self.get_component_array().unwrap(),
    //         comp_arr_ref_1: self.get_component_array().unwrap(),
    //     }
    // }
    // pub fn query<'a, T: ComponentsTuple>(&'a self) -> T::Query<'a> {
    //     T::query(self)
    // }
    // pub fn query2<'a, T0: 'static, T1: 'static>(&'a self) -> Query2<'a, T0, T1> {
    //     Query2 {
    //         comp_arr_ref_0: self.get_component_array().unwrap(),
    //         comp_arr_ref_1: self.get_component_array().unwrap(),
    //     }
    // }
}

// trait QuerySubject {}
//
// impl<T: Component> QuerySubject for T {}

pub struct Query<'w, T: Fetcherable> {
    // world: &'w mut World,
    fetch: T::Fetch<'w>,
}

impl<'w, T: Fetcherable> Query<'w, T> {
    fn new(world: &'w mut World) -> Self {
        let fetch = T::fetch_init(world);
        Self { fetch }
    }
}

pub trait Fetcherable {
    type Item<'w>;
    type Fetch<'w>;

    fn fetch_init<'w>(world: &'w mut World) -> Self::Fetch<'w>;
    fn fetch_next<'w>(fetch: &'w mut Self::Fetch<'w>) -> Self::Item<'w>;
}

impl<T: Component> Fetcherable for &T {
    type Item<'w> = &'w T;
    type Fetch<'w> = &'w ComponentArrayT<T>;

    fn fetch_init<'w>(world: &'w mut World) -> Self::Fetch<'w> {
        todo!()
    }

    fn fetch_next<'w>(fetch: &'w mut Self::Fetch<'w>) -> Self::Item<'w> {
        todo!()
    }
}

impl<T0: Fetcherable, T1: Fetcherable> Fetcherable for (T0, T1) {
    type Item<'w> = (T0::Item<'w>, T1::Item<'w>);
    type Fetch<'w> = (T0::Fetch<'w>, T1::Fetch<'w>);

    fn fetch_init<'w>(world: &'w mut World) -> Self::Fetch<'w> {
        todo!()
    }

    fn fetch_next<'w>(fetch: &'w mut Self::Fetch<'w>) -> Self::Item<'w> {
        todo!()
    }
}

// pub trait ComponentsTuple {
//     type RefsTuple<'a>;
//     type Query<'a>;
//     type ComponentArraysRefsTuple<'a>;
//
//     fn query<'a>(ecs: &'a Ecs) -> Self::Query<'a>;
// }
//
// impl<T0: 'static, T1: 'static> ComponentsTuple for (T0, T1) {
//     type RefsTuple<'a> = (&'a T0, &'a T1);
//     type Query<'a> = Query<'a, Self::ComponentArraysRefsTuple<'a>>;
//     type ComponentArraysRefsTuple<'a> = (
//         Ref<'a, ComponentArrayT<T0>>,
//         Ref<'a, ComponentArrayT<T1>>,
//     );
//
//     fn query<'a>(ecs: &'a Ecs) -> Self::Query<'a> {
//         Query2 {
//             comp_arr_refs: (
//                 ecs.get_component_array().unwrap(),
//                 ecs.get_component_array().unwrap(),
//             ),
//         }
//     }
// }
//
// pub struct Query<'a, T: ComponentsTuple> {
//     comp_arr_refs: T::ComponentArraysRefsTuple<'a>,
// }

// impl<'a, T: ComponentsTuple> Query<'a, T> {
//     pub fn
// }

// impl<'a, T0: 'static, T1: 'static> Query2<'a, T0, T1> {
//     pub fn iter(&'a self) -> ComponentIterator2<'a, T0, T1> {
//         ComponentIterator2 {
//             query: self,
//             cur_ent: Entity(0),
//         }
//     }
// }

// pub struct ComponentIterator2<'a, T0: 'static, T1: 'static> {
//     query: &'a Query2<'a, T0, T1>,
//     cur_ent: Entity,
// }
//
// impl<'a, T0: 'a + 'static, T1: 'a + 'static> Iterator for ComponentIterator2<'a, T0, T1> {
//     type Item = (Entity, &'a T0, &'a T1);
//
//     fn next(&mut self) -> Option<Self::Item> {
//         let query = self.query;
//         debug_assert_eq!(
//             query.comp_arr_ref_0.components.len(),
//             query.comp_arr_ref_1.components.len()
//         );
//         let len = query.comp_arr_ref_1.components.len();
//         loop {
//             let cur_ent = self.cur_ent;
//             let cur_ent_idx = self.cur_ent.0;
//
//             // End of the components
//             if (cur_ent_idx >= len) {
//                 break None;
//             }
//
//             self.cur_ent.0 += 1;
//
//             // SAFETY: we was checked bounds already
//             let c0 = unsafe { query.comp_arr_ref_0.components.get_unchecked(cur_ent_idx) };
//             let c1 = unsafe { query.comp_arr_ref_1.components.get_unchecked(cur_ent_idx) };
//
//             match (c0, c1) {
//                 (Some(c0), Some(c1)) => {
//                     break Some((cur_ent, c0, c1));
//                 }
//                 _ => {}
//             }
//         }
//     }
// }

//
// pub struct ComponentFetch<'a, T: 'static> {
//     component_array_ref: Ref<'a, ComponentArrayT<T>>,
// }
//
// pub trait Query {
//     type Item<'a>;
//     type Fetch<'a>;
//
//     fn fetch<'w>(fetch: &'w mut Self::Fetch<'w>, e: Entity) -> Self::Item<'w>;
// }
//
// pub trait Component: 'static {}
//
// impl<T: 'static> Component for T {}
//
// impl<T: Component> Query for T {
//     type Item<'a> = &'a Option<T>;
//     type Fetch<'a> = ComponentFetch<'a, T>;
//
//     fn fetch<'w>(fetch: &'w mut Self::Fetch<'w>, e: Entity) -> Self::Item<'w> {
//         fetch.component_array_ref.components.get(e.0).unwrap()
//     }
// }
//
// pub struct ComponentIterator<'a, T: Component + Query> {
//     fetch: T::Fetch<'a>,
//     cur_ent: Entity,
// }
//
// impl<'a, T: Component + Query> ComponentIterator<'a, T> {
//     pub fn new(fetch: T::Fetch<'a>) -> Self {
//         Self {
//             fetch,
//             cur_ent: Entity(0),
//         }
//     }
// }
//
// impl<'a, T: Component + Query> Iterator for ComponentIterator<'a, T> {
//     type Item = T::Item<'a>;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         let cur_ent = self.cur_ent;
//         self.cur_ent.0 += 1;
//         let g = &self.fetch;
//         g.
//         todo!()
//     }
// }

// impl<'a, T: 'static> Query<'a, T> {
//     pub fn new(component_array_ref: Ref<'a, ComponentArrayT<T>>) -> Self {
//         Self {
//             component_array_ref,
//         }
//     }
//
//     pub fn iterate(&'a self) -> ComponentIterator<'a, T> {
//         ComponentIterator::new(self)
//     }
// }
//
// pub struct ComponentIterator<'a, T: 'a + 'static> {
//     query: &'a Query<'a, T>,
//     cur_entity: Entity,
// }
//
// impl<'a, T: 'static> ComponentIterator<'a, T> {
//     pub fn new(query: &'a Query<'a, T>) -> Self {
//         Self {
//             query,
//             cur_entity: Entity(0),
//         }
//     }
// }
//
// impl<'a, T: 'a + 'static> Iterator for ComponentIterator<'a, T> {
//     type Item = (Entity, &'a T);
//
//     fn next(&mut self) -> Option<Self::Item> {
//         let ca = &self.query.component_array_ref.components;
//         loop {
//             let cur_entity = self.cur_entity;
//             self.cur_entity.0 += 1;
//             let comp = ca.get(cur_entity.0);
//             match comp {
//                 None => {
//                     break None;
//                 }
//                 Some(Some(comp)) => {
//                     break Some((cur_entity, comp));
//                 }
//                 Some(None) => {}
//             }
//         }
//     }
// }

// pub struct Fetch<'a, T: 'static>
// {
//     component_array: &'a ComponentArrayT<T>,
// }
//
//
// ($($name::Fetch<'w>,)*);

// trait Fetch<'a> {
//     type ComponentArrays;
//     type Items;
//
//     fn init_fetch(ecs: &Ecs) -> Self::ComponentArrays;
//     fn fetch(ca: &Self::ComponentArrays, e: Entity) -> &Option<Self::Items>;
// }
//
// impl<'a, T: 'static> Fetch<'a> for (T, ) {
//     type ComponentArrays = Ref<'a, ComponentArrayT<T>>;
//     type Items = T;
//
//     fn init_fetch(ecs: &Ecs) -> Self::ComponentArrays {
//         ecs.get_component_array::<T>().unwrap()
//     }
//
//     fn fetch(ca: &Self::ComponentArrays, e: Entity) -> &Option<Self::Items> {
//         &ca.components[e.0]
//     }
// }
//
// macro_rules! impl_fetch {
//     ($($type_name: ident, $name: ident),*) => {
//         impl<'a, $($type_name: 'static,)*> Fetch<'a> for ($($type_name, )*){
//             type Item = ($(&'a $type_name,)*);
//         }
//     };
// }
//
// // impl_fetch!(A0, a0, A1, a1);
//
// macro_rules! all_a {
//     ($macro_name: ident) => {
//         $macro_name!(A0, a0);
//         $macro_name!(A0, a0, A1, a1);
//         $macro_name!(A0, a0, A1, a1, A2, a2);
//         $macro_name!(A0, a0, A1, a1, A2, a2, A3, a3);
//         $macro_name!(A0, a0, A1, a1, A2, a2, A3, a3, A4, a4);
//         $macro_name!(A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5);
//         $macro_name!(A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6);
//         $macro_name!(A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6, A7, a7);
//         $macro_name!(A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5, A6, a6, A7, a7, A8, a8);
//     };
// }

// all_a!(impl_fetch);
