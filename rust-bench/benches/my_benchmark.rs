use criterion::{black_box, criterion_group, criterion_main, Criterion};
use libc::c_int;

#[repr(C)]
pub struct MyClass {
    pub x: c_int,
    pub y: c_int,
}

extern "C" {
    fn malloc_and_access(a: c_int, b: c_int) -> *mut MyClass;
    fn free_memory(ptr: *mut MyClass);
}

fn direct() {
    unsafe {
        let ptr = malloc_and_access(1,2);

        if ptr.is_null() {
            println!("malloc fail");
            return;
        }

        for _i in 0..10000 {
            let x = (*ptr).x;
            let y = (*ptr).y;
            black_box(x);
            black_box(y);
        }

        free_memory(ptr);
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

    pub fn gety(&self) -> i32 {
        unsafe {
            (*(self.ptr)).y
        }
    }
}

fn wrap() {
    unsafe {
        let ptr = malloc_and_access(1,2);

        if ptr.is_null() {
            println!("malloc fail");
            return;
        }

        let wrap = Box::new(Wrap{ptr});

        for _i in 0..10000 {
            let x = wrap.getx();
            let y = wrap.gety();
            black_box(x);
            black_box(y);
        }

        free_memory(ptr);
    }
}

fn benchmark_ffi_direct(c: &mut Criterion) {
    c.bench_function("ffi_direct", |b| {
        b.iter(|| {
            direct();
        })
    });
}

fn benchmark_ffi_wrap(c: &mut Criterion) {
    c.bench_function("ffi_wrap", |b| {
        b.iter(|| {
            wrap();
        })
    });
}

criterion_group!(benches, benchmark_ffi_direct, benchmark_ffi_wrap);
criterion_main!(benches);
