use constgrid::*;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::Rng;
const WIDTH: usize = 300;
const HEIGHT: usize = 200;

/// Some hard math operation to apply to the grid.
fn operation(difficulty: usize) -> impl Fn(Point) -> usize {
    let f = black_box(|difficulty| {
        move |Point { mut x, mut y }| {
            for _ in 0..difficulty {
                y = x + y;
                x = if difficulty >= 1 && x % 2 == 0 {
                    x.pow(2) + y
                } else {
                    x + y
                }
            }
            x + y
        }
    });
    f(difficulty)
}

/// Benchmark the `set_all_parallel` method of Gridlike.
fn set_grid_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Set");
    let difficulties = vec![1, 5];
    for d in difficulties {
        group.bench_with_input(BenchmarkId::new("1D Array", d), &d, |b, d| {
            let mut g: array1d::Grid<usize, WIDTH, HEIGHT> = Default::default();
            b.iter(|| g.set_all_parallel(operation(d.clone())));
        });
        group.bench_with_input(BenchmarkId::new("2D Array", d), &d, |b, d| {
            let mut g: array2d::Grid<usize, WIDTH, HEIGHT> = Default::default();
            b.iter(|| g.set_all_parallel(operation(d.clone())));
        });
        group.bench_with_input(BenchmarkId::new("1D Vec", d), &d, |b, d| {
            let mut g = vec1d::Grid::new(WIDTH, HEIGHT);
            b.iter(|| g.set_all_parallel(operation(d.clone())));
        });
        group.bench_with_input(BenchmarkId::new("2D Vec", d), &d, |b, d| {
            let mut g = vec2d::Grid::new(WIDTH, HEIGHT);
            b.iter(|| g.set_all_parallel(operation(d.clone())));
        });
    }
    group.finish();
}
/// Benchmark the `get` method of Gridlike with random access.
fn get_grid_bench_random(c: &mut Criterion) {
    let mut group = c.benchmark_group("GetRandom");
    let point = || Point {
        x: rand::thread_rng().gen_range(0..WIDTH),
        y: rand::thread_rng().gen_range(0..HEIGHT),
    };
    for d in &[1, 10, 100, 1000] {
        group.bench_with_input(BenchmarkId::new("1D Array", d), &d, |b, d| {
            let mut g: array1d::Grid<usize, WIDTH, HEIGHT> = Default::default();
            g.set_all_parallel(operation(1));
            b.iter(|| {
                for _ in 0..**d {
                    black_box(g.get(point()));
                }
            });
        });
        group.bench_with_input(BenchmarkId::new("2D Array", d), &d, |b, d| {
            let mut g: array2d::Grid<usize, WIDTH, HEIGHT> = Default::default();
            g.set_all_parallel(operation(1));
            b.iter(|| {
                for _ in 0..**d {
                    black_box(g.get(point()));
                }
            });
        });
        group.bench_with_input(BenchmarkId::new("1D Vec", d), &d, |b, d| {
            let mut g = vec1d::Grid::new(WIDTH, HEIGHT);
            g.set_all_parallel(operation(1));
            b.iter(|| {
                for _ in 0..**d {
                    black_box(g.get(point()));
                }
            });
        });
        group.bench_with_input(BenchmarkId::new("2D Vec", d), &d, |b, d| {
            let mut g = vec2d::Grid::new(WIDTH, HEIGHT);
            g.set_all_parallel(operation(1));
            b.iter(|| {
                for _ in 0..**d {
                    black_box(g.get(point()));
                }
            });
        });
    }
    group.finish();
}
/// Benchmark the `get` method of Gridlike, accessing elements in a predictable order.
fn get_grid_bench_order(c: &mut Criterion) {
    let mut group = c.benchmark_group("GetOrder");
    for d in &[1, 50, 100, 200] {
        group.bench_with_input(BenchmarkId::new("1D Array", d), &d, |b, d| {
            let mut g: array1d::Grid<usize, WIDTH, HEIGHT> = Default::default();
            g.set_all_parallel(operation(1));
            b.iter(|| {
                for x in 0..**d {
                    for y in 0..**d {
                        black_box(g.get(Point { x, y }));
                    }
                }
            });
        });
        group.bench_with_input(BenchmarkId::new("2D Array", d), &d, |b, d| {
            let mut g: array2d::Grid<usize, WIDTH, HEIGHT> = Default::default();
            g.set_all_parallel(operation(1));
            b.iter(|| {
                for x in 0..**d {
                    for y in 0..**d {
                        black_box(g.get(Point { x, y }));
                    }
                }
            });
        });
        group.bench_with_input(BenchmarkId::new("1D Vec", d), &d, |b, d| {
            let mut g = vec1d::Grid::new(WIDTH, HEIGHT);
            g.set_all_parallel(operation(1));
            b.iter(|| {
                for x in 0..**d {
                    for y in 0..**d {
                        black_box(g.get(Point { x, y }));
                    }
                }
            });
        });
        group.bench_with_input(BenchmarkId::new("2D Vec", d), &d, |b, d| {
            let mut g = vec2d::Grid::new(WIDTH, HEIGHT);
            g.set_all_parallel(operation(1));
            b.iter(|| {
                for x in 0..**d {
                    for y in 0..**d {
                        black_box(g.get(Point { x, y }));
                    }
                }
            });
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    set_grid_bench,
    get_grid_bench_order,
    get_grid_bench_random
);
criterion_main!(benches);
