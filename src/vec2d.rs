use crate::Gridlike;

use super::Point;
pub struct Grid<T> {
    array: Vec<Vec<T>>,
    pub width: usize,
    pub height: usize,
}

impl<T> Grid<T> {
    pub fn new(width: usize, height: usize) -> Self
    where
        T: Default + Copy,
    {
        let mut array = Vec::with_capacity(height);
        for _ in 0..height {
            array.push([T::default()].repeat(width * height));
        }
        Self {
            array,
            width,
            height,
        }
    }
}

impl<T> Gridlike<T> for Grid<T> {
    fn get(&self, p: &Point) -> &T {
        &self.array[p.y][p.x]
    }

    fn set_all_parallel<F>(&mut self, setter: F)
    where
        F: Send + Sync + Fn(Point) -> T,
        T: Send,
    {
        use rayon::prelude::*;
        self.array.par_iter_mut().enumerate().for_each(|(y, row)| {
            for (x, item) in row.iter_mut().enumerate() {
                *item = setter(Point { x, y });
            }
        });
    }
}
