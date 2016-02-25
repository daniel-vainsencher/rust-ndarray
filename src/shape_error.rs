use std::fmt;
use std::error::Error;
use super::{
    Ix,
    Dimension,
};

/// An error that can be produced by `.into_shape()`
#[derive(Clone, Debug)]
pub enum ShapeError {
    /// incompatible shapes in reshape, (from, to)
    IncompatibleShapes(Box<[Ix]>, Box<[Ix]>),
    /// incompatible layout: not contiguous
    IncompatibleLayout,
    /// Dimension too large (shape)
    DimensionTooLarge(Box<[Ix]>),
}

#[inline(never)]
#[cold]
pub fn incompatible_shapes<D, E>(a: &D, b: &E) -> ShapeError
    where D: Dimension,
          E: Dimension
{
    ShapeError::IncompatibleShapes(a.slice().to_vec().into_boxed_slice(),
                                   b.slice().to_vec().into_boxed_slice())
}


impl Error for ShapeError {
    fn description(&self) -> &str {
        match *self {
            ShapeError::IncompatibleShapes(..) => "incompatible shapes",
            ShapeError::IncompatibleLayout => "incompatible layout (not contiguous)",
            ShapeError::DimensionTooLarge(..) => "dimension too large",
        }
    }
}

impl fmt::Display for ShapeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ShapeError::IncompatibleShapes(ref a, ref b) => {
                write!(f, "incompatible shapes, attempted from: {:?}, to: {:?}",
                       a, b)
            }
            ShapeError::IncompatibleLayout => {
                write!(f, "{}", self.description())
            }
            ShapeError::DimensionTooLarge(ref a) => {
                write!(f, "dimension too large in shape {:?}", a)
            }
        }
    }
}
