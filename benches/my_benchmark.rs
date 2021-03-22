use constgrid::*;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
const WIDTH: usize = 300;
const HEIGHT: usize = 200;

fn set_grid_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("SetGrid");
    let operation = black_box(|difficulty| {
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
    let difficulties = vec![1, 5, 10];
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

criterion_group!(benches, set_grid_bench);
criterion_main!(benches);
