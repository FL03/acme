/*
    Appellation: tensor <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::actions::iter::StrideIter;
use crate::data::Layout;
use crate::error::{TensorError, TensorResult};
use crate::ops::{BackpropOp, TensorExpr};
use crate::prelude::{TensorId, TensorKind};
use crate::shape::{IntoShape, Rank, Shape, Stride};

#[cfg(not(feature = "std"))]
use alloc::vec::{self, Vec};
use core::iter::Map;
use core::ops::{Index, IndexMut};
use core::slice::Iter as SliceIter;
#[cfg(feature = "std")]
use std::vec;

pub(crate) fn create_with<T>(
    kind: impl Into<TensorKind>,
    op: impl Into<BackpropOp<T>>,
    shape: impl IntoShape,
    data: Vec<T>,
) -> TensorBase<T> {
    TensorBase {
        id: TensorId::new(),
        data,
        kind: kind.into(),
        layout: Layout::contiguous(shape),
        op: op.into(),
    }
}
#[allow(dead_code)]
pub(crate) fn from_scalar_with_op<T>(
    kind: impl Into<TensorKind>,
    op: TensorExpr<T>,
    data: T,
) -> TensorBase<T> {
    create_with(
        kind.into(),
        BackpropOp::new(op),
        Shape::scalar(),
        vec![data],
    )
}

pub(crate) fn from_vec_with_kind<T>(
    kind: impl Into<TensorKind>,
    shape: impl IntoShape,
    data: Vec<T>,
) -> TensorBase<T> {
    create_with(kind, BackpropOp::none(), shape, data)
}

pub(crate) fn from_vec_with_op<T>(
    kind: impl Into<TensorKind>,
    op: TensorExpr<T>,
    shape: impl IntoShape,
    data: Vec<T>,
) -> TensorBase<T> {
    create_with(kind.into(), BackpropOp::new(op), shape, data)
}

#[derive(Clone, Debug, Hash)]
pub struct TensorBase<T = f64> {
    pub(crate) id: TensorId,
    pub(crate) data: Vec<T>,
    pub(crate) kind: TensorKind,
    pub(crate) layout: Layout,
    pub(crate) op: BackpropOp<T>,
}

impl<T> TensorBase<T> {
    pub fn new(kind: TensorKind, shape: impl IntoShape) -> Self {
        let shape = shape.into_shape();
        let data = Vec::with_capacity(shape.size());
        Self {
            id: TensorId::new(),
            data,
            kind,
            layout: Layout::contiguous(shape),
            op: BackpropOp::none(),
        }
    }
    /// Create a new tensor from an iterator.
    pub fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Self::from_vec(Vec::from_iter(iter))
    }
    /// Create a new tensor from a scalar value.
    pub fn from_scalar(value: T) -> Self {
        Self {
            id: TensorId::new(),
            data: vec![value],
            kind: TensorKind::default(),
            layout: Layout::contiguous(()),
            op: None.into(),
        }
    }
    /// Create a new tensor from an iterator, with a particular shape.
    pub fn from_shape_iter<I>(shape: impl IntoShape, iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Self::from_shape_vec(shape, Vec::from_iter(iter))
    }
    /// Create a new tensor from a [Vec], with a specified [shape](Shape).
    pub fn from_shape_vec(shape: impl IntoShape, data: Vec<T>) -> Self {
        Self {
            id: TensorId::new(),
            data,
            kind: TensorKind::default(),
            layout: Layout::contiguous(shape),
            op: BackpropOp::none(),
        }
    }
    /// Create a new, one-dimensional tensor from a [Vec].
    pub fn from_vec(data: Vec<T>) -> Self {
        let shape = Shape::from(data.len());
        Self {
            id: TensorId::new(),
            data,
            kind: TensorKind::default(),
            layout: Layout::contiguous(shape),
            op: BackpropOp::none(),
        }
    }
    /// Return a reference to the tensor's data.
    pub fn as_slice(&self) -> &[T] {
        &self.data
    }
    /// Return a mutable reference to the tensor's data.
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.data
    }
    /// Detach the computational graph from the tensor
    pub fn detach(&self) -> Self
    where
        T: Clone,
    {
        if self.op.is_none() && !self.is_variable() {
            self.clone()
        } else {
            Self {
                id: self.id,
                kind: self.kind,
                layout: self.layout.clone(),
                op: BackpropOp::none(),
                data: self.data.clone(),
            }
        }
    }
    /// Returns a reference to the first element of the tensor.
    pub fn first(&self) -> Option<&T> {
        let pos = vec![0; *self.rank()];
        self.get(pos)
    }
    /// Returns a mutable reference to the first element of the tensor.
    pub fn first_mut(&mut self) -> Option<&mut T> {
        let pos = vec![0; *self.rank()];
        self.get_mut(pos)
    }
    /// Returns the data at the specified index.
    pub fn get(&self, index: impl AsRef<[usize]>) -> Option<&T> {
        let i = self.layout.index(index);
        self.data().get(i)
    }
    /// Returns a mutable reference to the data at the specified index.
    pub fn get_mut(&mut self, index: impl AsRef<[usize]>) -> Option<&mut T> {
        let i = self.layout.index(index);
        self.data_mut().get_mut(i)
    }
    /// Returns the unique identifier of the tensor.
    pub const fn id(&self) -> TensorId {
        self.id
    }
    /// Returns true if the tensor is contiguous.
    pub fn is_contiguous(&self) -> bool {
        self.layout().is_contiguous()
    }
    /// Returns true if the tensor is empty.
    pub fn is_empty(&self) -> bool {
        self.data().is_empty()
    }
    /// A function to check if the tensor is a scalar
    pub fn is_scalar(&self) -> bool {
        *self.rank() == 0
    }
    /// Returns true if the tensor is a square matrix.
    pub fn is_square(&self) -> bool {
        self.shape().is_square()
    }
    /// A function to check if the tensor is a variable
    pub const fn is_variable(&self) -> bool {
        self.kind().is_variable()
    }
    /// Return an iterator over the tensor
    pub fn iter(&self) -> StrideIter<'_, T> {
        StrideIter::new(self)
    }
    /// Get the kind of the tensor
    pub const fn kind(&self) -> TensorKind {
        self.kind
    }
    /// Get a reference to the last element of the tensor
    pub fn last(&self) -> Option<&T> {
        let pos = self.shape().get_final_position();
        self.get(pos)
    }
    /// Get a mutable reference to the last element of the tensor
    pub fn last_mut(&mut self) -> Option<&mut T> {
        let pos = self.shape().get_final_position();
        self.get_mut(pos)
    }
    /// Get a reference to the [Layout] of the tensor
    pub const fn layout(&self) -> &Layout {
        &self.layout
    }
    /// Get the number of columns in the tensor
    pub fn ncols(&self) -> usize {
        self.shape().ncols()
    }
    /// Get the number of rows in the tensor
    pub fn nrows(&self) -> usize {
        self.shape().nrows()
    }
    /// Get a reference to the operation of the tensor
    pub const fn op(&self) -> &BackpropOp<T> {
        &self.op
    }
    /// Get a reference to the operation of the tensor
    pub fn op_view(&self) -> BackpropOp<&T> {
        self.op().view()
    }
    /// Get an owned reference to the [Rank] of the tensor
    pub fn rank(&self) -> Rank {
        self.shape().rank()
    }
    /// An owned reference of the tensors [Shape]
    pub fn shape(&self) -> &Shape {
        self.layout().shape()
    }
    /// Returns the number of elements in the tensor.
    pub fn size(&self) -> usize {
        self.layout().size()
    }
    /// Get a reference to the stride of the tensor
    pub fn stride(&self) -> &Stride {
        self.layout().stride()
    }
    /// Turn the tensor into a scalar
    /// If the tensor has a rank greater than 0, this will return an error
    pub fn to_scalar(&self) -> TensorResult<&T> {
        if !self.is_scalar() {
            return Err(TensorError::NotScalar);
        }
        Ok(self.first().unwrap())
    }
    /// Turn the tensor into a one-dimensional vector
    pub fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        self.data().to_vec()
    }
    /// Changes the kind of tensor to a variable
    pub fn variable(mut self) -> Self {
        self.kind = TensorKind::Variable;
        self
    }
    ///
    pub unsafe fn with_layout_unchecked(mut self, layout: Layout) -> Self {
        self.layout = layout;
        self
    }

    pub fn with_op(mut self, op: BackpropOp<T>) -> Self {
        self.op = op;
        self
    }

    pub unsafe fn with_shape_unchecked(mut self, shape: impl IntoShape) -> Self {
        self.layout = Layout::contiguous(shape);
        self
    }
}

impl<T> TensorBase<T> {
    pub fn to_owned(&self) -> TensorBase<T>
    where
        T: Clone,
    {
        self.clone()
    }

    pub fn view<'a>(&'a self) -> TensorBase<&'a T> {
        let store = self.data.iter().collect();
        TensorBase {
            id: self.id,
            kind: self.kind,
            layout: self.layout.clone(),
            op: self.op.view(),
            data: store,
        }
    }
}
// Inernal Methods
#[allow(dead_code)]
impl<T> TensorBase<T> {
    pub(crate) fn data(&self) -> &Vec<T> {
        &self.data
    }

    pub(crate) fn data_mut(&mut self) -> &mut Vec<T> {
        &mut self.data
    }

    pub(crate) fn get_by_index(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    pub(crate) fn map<'a, F>(&'a self, f: F) -> Map<SliceIter<'a, T>, F>
    where
        F: FnMut(&'a T) -> T,
        T: 'a + Clone,
    {
        self.data.iter().map(f)
    }

    pub(crate) fn mapv<F>(&self, f: F) -> TensorBase<T>
    where
        F: Fn(T) -> T,
        T: Copy,
    {
        let store = self.data.iter().copied().map(f).collect();
        TensorBase {
            id: TensorId::new(),
            kind: self.kind,
            layout: self.layout.clone(),
            op: self.op.clone(),
            data: store,
        }
    }
}

impl<'a, T> AsRef<TensorBase<T>> for TensorBase<&'a T> {
    fn as_ref(&self) -> &TensorBase<T> {
        unsafe { &*(self as *const TensorBase<&'a T> as *const TensorBase<T>) }
    }
}

impl<Idx, T> Index<Idx> for TensorBase<T>
where
    Idx: AsRef<[usize]>,
{
    type Output = T;

    fn index(&self, index: Idx) -> &Self::Output {
        let i = self.layout().index(index);
        &self.data[i]
    }
}

impl<Idx, T> IndexMut<Idx> for TensorBase<T>
where
    Idx: AsRef<[usize]>,
{
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        let i = self.layout().index(index);
        &mut self.data[i]
    }
}

impl<T> Eq for TensorBase<T> where T: Eq {}

impl<T> PartialEq for TensorBase<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.layout == other.layout && self.data == other.data
    }
}
