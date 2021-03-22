use crate::Gridlike;

use super::Point;
pub struct Grid<T> {
    array: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T> Grid<T> {
    pub fn new(width: usize, height: usize) -> Self
    where
        T: Default + Copy,
    {
        Self {
            array: [T::default()].repeat(width * height),
            width,
            height,
        }
    }
}

impl<T> Gridlike<T> for Grid<T> {
    fn get(&self, p: &Point) -> &T {
        &self.array[p.y * self.width + p.x]
    }

    fn set_all_parallel<F>(&mut self, setter: F)
    where
        F: Send + Sync + Fn(Point) -> T,
        T: Send,
    {
        use rayon::prelude::*;
        let width = self.width;
        self.array.par_iter_mut().enumerate().for_each(|(i, item)| {
            *item = setter(Point {
                x: i % width,
                y: i / width,
            });
        });
    }
    fn width(&self) -> usize {
        self.width
    }
    fn height(&self) -> usize {
        self.height
    }
}
