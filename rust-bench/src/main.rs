use criterion::{black_box, Criterion};
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

fn rust_direct() {
    unsafe {
        let ptr = malloc_and_access(1,2.2);

        for _i in 0..10000 {
            let x = (*ptr).x;
            let y = (*ptr).y;
            black_box(x);
            black_box(y);
        }

        free_memory(ptr);
    }
}

fn rust_wrap() {
    unsafe {
        let ptr = malloc_and_access(1,2.2);
        let wrap = Wrap{ptr};

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

fn benchmark_rust_direct(c: &mut Criterion) {
    c.bench_function("纯rust语言的直接调用", |b| {
        b.iter(|| {
            rust_direct();
        })
    });
}

fn benchmark_rust_wrap(c: &mut Criterion) {
    c.bench_function("纯rust语言的封装调用", |b| {
        b.iter(|| {
            rust_wrap();
        })
    });
}

fn main() {
    // 加了without_plots才能在cmd打印出结果
    let mut criterion = Criterion::default().without_plots();

    // 用于模拟: 频繁申请+释放JSObject的场景
    // 申请和释放涉及跨语言交互
    benchmark_ffi_direct(&mut criterion);
    benchmark_ffi_wrap(&mut criterion);

    // 用于模拟: 申请一个JSObject后, 频繁访问对象属性的场景
    // 通过解引用裸指针, 实现rust直接访问C内存, 不涉及跨语言交互
    benchmark_rust_direct(&mut criterion);
    benchmark_rust_wrap(&mut criterion);

    criterion.final_summary();  // 打印结果
}
