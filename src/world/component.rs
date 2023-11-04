use std::any::TypeId;

pub trait Component: 'static {
    fn get_type_id() -> TypeId;
}

impl<T: 'static> Component for T {
    fn get_type_id() -> TypeId {
        TypeId::of::<T>()
    }
}
