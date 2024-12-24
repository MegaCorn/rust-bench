use criterion::{black_box, criterion_group, criterion_main, Criterion};

extern "C" {
    fn square(x: i32) -> i32;
}

fn _square(x: i32) -> i32 {
    x * x
}

fn benchmark_ffi_square(c: &mut Criterion) {
    c.bench_function("ffi_square", |b| {
        b.iter(|| {
            unsafe { black_box(square(3)) };
        })
    });
}

fn benchmark_rust_square(c: &mut Criterion) {
    c.bench_function("rust_square", |b| {
        b.iter(|| {
            black_box(_square(3));
        })
    });
}

criterion_group!(benches, benchmark_ffi_square, benchmark_rust_square);
criterion_main!(benches);
