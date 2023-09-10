pub trait DefaultConstruct {
    fn new() -> Self;
}

#[macro_export]
macro_rules! impl_default {
    ($Type:ty,
    $Default:expr) => {
        impl DefaultConstruct for $Type {
            fn new() -> Self {
                $Default
            }
        }
    };
}

impl_default!(i8, 0i8);
impl_default!(i16, 0i16);
impl_default!(i32, 0i32);
impl_default!(i64, 0i64);
impl_default!(i128, 0i128);
impl_default!(isize, 0isize);

impl_default!(u8, 0u8);
impl_default!(u16, 0u16);
impl_default!(u32, 0u32);
impl_default!(u64, 0u64);
impl_default!(u128, 0u128);
impl_default!(usize, 0usize);

impl_default!(f32, 0f32);
impl_default!(f64, 0f64);
