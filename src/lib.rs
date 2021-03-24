#![feature(const_evaluatable_checked)]
#![feature(const_generics)]

pub mod array1d;
pub mod array2d;
pub mod vec1d;
pub mod vec2d;

/// A point used to index a 2D grid.
#[derive(Clone, Copy)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

/// A container which stores elements at 2D points.
pub trait Gridlike<T> {
    fn width(&self) -> usize;
    fn height(&self) -> usize;

    /// Get the element at the given point.
    fn get(&self, p: Point) -> &T;

    /// Set all elements of the grid, using a setter function.
    /// The setter function takes a point and returns the value which should be
    /// assigned to the grid at that point.
    fn set_all_parallel<F>(&mut self, setter: F)
    where
        F: Send + Sync + Fn(Point) -> T,
        T: Send;
}
