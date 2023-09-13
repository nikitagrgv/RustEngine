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


