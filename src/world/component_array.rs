use crate::world::{Component, Entity};
use std::any::Any;
use std::cell::{Cell, UnsafeCell};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;

/// ComponentArray
pub(in crate::world) trait ComponentArray {
    fn push_none(&mut self);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// ComponentArrayCell
pub struct CACell<T: Component> {
    borrow: Cell<BorrowFlag>,
    components: UnsafeCell<Vec<Option<T>>>,
}

impl<T: Component> CACell<T> {
    pub fn set_component(&mut self, component: T, e: Entity) {
        self.components.get_mut()[e.to_num()] = Some(component).into();
    }

    pub fn remove_component(&mut self, entity: Entity) {
        self.components.get_mut()[entity.to_num()] = None.into();
    }
}

impl<T: Component> ComponentArray for CACell<T> {
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

/// BorrowFlag
type BorrowFlag = isize;
const UNUSED: BorrowFlag = 0;
fn is_writing(x: BorrowFlag) -> bool {
    x < UNUSED
}
fn is_reading(x: BorrowFlag) -> bool {
    x > UNUSED
}

/// CABorrow
struct CABorrow<'b> {
    borrow: &'b Cell<BorrowFlag>,
}

impl<'b> CABorrow<'b> {
    #[inline]
    fn new(borrow: &'b Cell<BorrowFlag>) -> Option<CABorrow<'b>> {
        let b = borrow.get().wrapping_add(1);
        if !is_reading(b) {
            None
        } else {
            borrow.set(b);
            Some(CABorrow { borrow })
        }
    }
}

impl Drop for CABorrow<'_> {
    #[inline]
    fn drop(&mut self) {
        let borrow = self.borrow.get();
        debug_assert!(is_reading(borrow));
        self.borrow.set(borrow - 1);
    }
}

impl Clone for CABorrow<'_> {
    #[inline]
    fn clone(&self) -> Self {
        let borrow = self.borrow.get();
        debug_assert!(is_reading(borrow));
        assert!(borrow != isize::MAX);
        self.borrow.set(borrow + 1);
        CABorrow {
            borrow: self.borrow,
        }
    }
}

/// CABorrowMut
struct CABorrowMut<'b> {
    borrow: &'b Cell<BorrowFlag>,
}

impl Drop for CABorrowMut<'_> {
    #[inline]
    fn drop(&mut self) {
        let borrow = self.borrow.get();
        debug_assert!(is_writing(borrow));
        self.borrow.set(borrow + 1);
    }
}

impl<'b> CABorrowMut<'b> {
    #[inline]
    fn new(borrow: &'b Cell<BorrowFlag>) -> Option<CABorrowMut<'b>> {
        match borrow.get() {
            UNUSED => {
                borrow.set(UNUSED - 1);
                Some(CABorrowMut { borrow })
            }
            _ => None,
        }
    }

    #[inline]
    fn clone(&self) -> CABorrowMut<'b> {
        let borrow = self.borrow.get();
        debug_assert!(is_writing(borrow));
        assert!(borrow != isize::MIN);
        self.borrow.set(borrow - 1);
        CABorrowMut {
            borrow: self.borrow,
        }
    }
}

/// CARef
pub struct CARef<'b, T: Component + 'b> {
    value: NonNull<Vec<Option<T>>>,
    borrow: CABorrow<'b>,
}

impl<T: Component> Deref for CARef<'_, T> {
    type Target = Vec<Option<T>>;

    fn deref(&self) -> &Self::Target {
        unsafe { self.value.as_ref() }
    }
}

/// CARefMut
pub struct CARefMut<'b, T: Component + 'b> {
    value: NonNull<Vec<Option<T>>>,
    borrow: CABorrowMut<'b>,
    marker: PhantomData<&'b mut T>,
}

impl<'b, T: Component + 'b> CARefMut<'b, T> {
    pub unsafe fn deref_mut_unsafe(&self) -> &mut Vec<Option<T>> {
        unsafe { NonNull::new_unchecked(self.value.as_ptr()).as_mut() }
    }
}

impl<T: Component> Deref for CARefMut<'_, T> {
    type Target = Vec<Option<T>>;

    fn deref(&self) -> &Self::Target {
        unsafe { self.value.as_ref() }
    }
}

impl<T: Component> DerefMut for CARefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.value.as_mut() }
    }
}

/// CACell
impl<T: Component> CACell<T> {
    pub fn new() -> Self {
        Self {
            borrow: Cell::new(UNUSED),
            components: UnsafeCell::new(Vec::new()),
        }
    }

    pub fn borrow(&self) -> CARef<'_, T> {
        self.try_borrow().expect("already mutably borrowed")
    }

    pub fn borrow_mut(&self) -> CARefMut<'_, T> {
        self.try_borrow_mut().expect("already borrowed")
    }

    pub fn try_borrow(&self) -> Option<CARef<'_, T>> {
        match CABorrow::new(&self.borrow) {
            Some(b) => {
                let value = unsafe { NonNull::new_unchecked(self.components.get()) };
                Some(CARef { value, borrow: b })
            }
            None => None,
        }
    }

    pub fn try_borrow_mut(&self) -> Option<CARefMut<'_, T>> {
        match CABorrowMut::new(&self.borrow) {
            Some(b) => {
                let value = unsafe { NonNull::new_unchecked(self.components.get()) };
                Some(CARefMut {
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
