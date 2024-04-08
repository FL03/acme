/*
    Appellation: iterator <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::LayoutIter;
use crate::TensorBase;
use core::marker::PhantomData;
use core::ptr;

pub struct Iter<'a, T> {
    pos: LayoutIter,
    // ptr: *const T,
    scope: Option<&'a T>,
    tensor: &'a TensorBase<T>,
}

impl<'a, T> Iter<'a, T> {
    pub fn new(tensor: &'a TensorBase<T>) -> Self {
        Self {
            pos: tensor.layout().iter(),
            // ptr: tensor.as_ptr(),
            scope: None,
            tensor,
        }
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T> {
    fn len(&self) -> usize {
        self.tensor.size()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.pos.next()?;
        self.scope = self.tensor.get_by_index(pos.index());
        self.scope
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let pos = self.pos.next_back()?;
        self.scope = self.tensor.get_by_index(pos.index());
        self.scope
    }
}

// impl<'a, T> Iterator for Iter<'a, T> {
//     type Item = &'a T;

//     fn next(&mut self) -> Option<Self::Item> {
//         let (_pos, idx) = self.pos.next()?;
//         let item = self.tensor.get_by_index(idx)?;
//         self.ptr = ptr::from_ref(item);
//         unsafe { self.ptr.as_ref() }
//     }
// }

// impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
//     fn next_back(&mut self) -> Option<Self::Item> {
//         let (_pos, idx) = self.pos.next_back()?;
//         let item = self.tensor.get_by_index(idx)?;
//         self.ptr = ptr::from_ref(item);
//         unsafe { self.ptr.as_ref() }
//     }
// }

impl<'a, T> From<&'a TensorBase<T>> for Iter<'a, T> {
    fn from(tensor: &'a TensorBase<T>) -> Self {
        Self::new(tensor)
    }
}

pub struct IterMut<'a, T: 'a> {
    positions: LayoutIter,
    ptr: *mut T,
    tensor: &'a mut TensorBase<T>,
    _marker: PhantomData<&'a mut T>,
}
impl<'a, T> IterMut<'a, T> {
    pub(crate) fn new(tensor: &'a mut TensorBase<T>) -> Self {
        Self {
            positions: tensor.layout().iter(),
            ptr: tensor.as_mut_ptr(),
            tensor,
            _marker: PhantomData,
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.positions.next()?;
        let elem = self.tensor.get_mut_by_index(pos.index())?;

        self.ptr = ptr::from_mut(elem);
        unsafe { self.ptr.as_mut() }
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let pos = self.positions.next_back()?;
        let elem = self.tensor.get_mut_by_index(pos.index())?;

        self.ptr = ptr::from_mut(elem);
        unsafe { self.ptr.as_mut() }
    }
}
