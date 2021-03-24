use crate::Gridlike;

use super::Point;
pub struct Grid<T, const W: usize, const H: usize>
where
    [T; W * H]: Sized,
{
    array: [T; W * H],
}

impl<T, const W: usize, const H: usize> Default for Grid<T, W, H>
where
    [T; W * H]: Sized,
    T: Default + Copy,
{
    fn default() -> Self {
        Self {
            array: [T::default(); W * H],
        }
    }
}

impl<T, const W: usize, const H: usize> Gridlike<T> for Grid<T, W, H>
where
    [T; W * H]: Sized,
{
    fn width(&self) -> usize {
        W
    }

    fn height(&self) -> usize {
        H
    }

    fn get(&self, p: Point) -> &T {
        &self.array[p.y * W + p.x]
    }
    fn set_all_parallel<F>(&mut self, setter: F)
    where
        F: Send + Sync + Fn(Point) -> T,
        T: Send,
    {
        use rayon::prelude::*;
        self.array.par_iter_mut().enumerate().for_each(|(i, item)| {
            *item = setter(Point { x: i % W, y: i / W });
        });
    }
}
