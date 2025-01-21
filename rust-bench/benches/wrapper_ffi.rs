use criterion::{black_box, criterion_group, criterion_main, Criterion};
use libc::{c_int, c_float};

#[repr(C)]
pub struct MyClass {
    pub x: c_int,
    pub y: c_float,
}

extern "C" {
    fn malloc_and_access(a: c_int, b: c_float) -> *mut MyClass;
    fn free_memory(ptr: *mut MyClass);
}

fn ffi_direct() {
    unsafe {
        for _i in 0..10000 {
            let ptr = malloc_and_access(1,2.2);
            let x = (*ptr).x;
            let y = (*ptr).y;
            black_box(x);
            black_box(y);
            free_memory(ptr);
        }
    }
}

struct Wrap {
    pub ptr: *mut MyClass,
}

impl Wrap {
    pub fn getx(&self) -> i32 {
        unsafe {
            (*(self.ptr)).x
        }
    }

    pub fn gety(&self) -> f32 {
        unsafe {
            (*(self.ptr)).y
        }
    }
}

fn ffi_wrap() {
    unsafe {
        for _i in 0..10000 {
            let ptr = malloc_and_access(1,2.2);
            let wrap = Wrap{ptr};
            let x = wrap.getx();
            let y = wrap.gety();
            black_box(x);
            black_box(y);
            free_memory(ptr);
        }
    }
}

fn benchmark_ffi_direct(c: &mut Criterion) {
    c.bench_function("发生了ffi的直接调用", |b| {
        b.iter(|| {
            ffi_direct();
        })
    });
}

fn benchmark_ffi_wrap(c: &mut Criterion) {
    c.bench_function("发生了ffi的封装调用", |b| {
        b.iter(|| {
            ffi_wrap();
        })
    });
}

criterion_group!(benches, benchmark_ffi_direct, benchmark_ffi_wrap);
criterion_main!(benches);
