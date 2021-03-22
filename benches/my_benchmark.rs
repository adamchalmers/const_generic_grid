use constgrid::*;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
const WIDTH: usize = 300;
const HEIGHT: usize = 200;

fn compare_grid_impls(c: &mut Criterion) {
    let mut group = c.benchmark_group("Grid");
    let operation = black_box(|Point { x, y }| x.pow(2) + y);
    group.bench_function(BenchmarkId::new("1D Array", ""), |b| {
        let mut g: array1d::Grid<usize, WIDTH, HEIGHT> = Default::default();
        b.iter(|| g.set_all_parallel(operation));
    });
    group.bench_function(BenchmarkId::new("2D Array", ""), |b| {
        let mut g: array2d::Grid<usize, WIDTH, HEIGHT> = Default::default();
        b.iter(|| g.set_all_parallel(operation));
    });
    group.bench_function(BenchmarkId::new("1D Vec", ""), |b| {
        let mut g = vec1d::Grid::new(WIDTH, HEIGHT);
        b.iter(|| g.set_all_parallel(operation));
    });
    group.bench_function(BenchmarkId::new("2D Vec", ""), |b| {
        let mut g = vec2d::Grid::new(WIDTH, HEIGHT);
        b.iter(|| g.set_all_parallel(operation));
    });
    group.finish();
}

criterion_group!(benches, compare_grid_impls);
criterion_main!(benches);
