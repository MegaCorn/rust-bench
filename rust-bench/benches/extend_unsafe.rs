use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::mem;

struct Node {
    count: i32,
}

impl Node {
    fn new() -> Self {
        Node {
            count: 1,
        }
    }

    fn method(&mut self) -> i32 {
        self.count += 1;
        self.count
    }
}

struct Element {
    base: Node,
}

impl Element {
    fn new() -> Self {
        Element {
            base: Node::new(),
        }
    }
}

struct DiveElement {
    base: Element,
}

impl DiveElement {
    fn new() -> Self {
        DiveElement {
            base: Element::new()
        }
    }
}

fn pointer() {
    let mut div = DiveElement::new();

    unsafe {
        for _i in 0..10000 {
            let div_ptr: *mut DiveElement = &mut div;
            let node_ptr: *mut Node = mem::transmute(div_ptr);
            let count = (*node_ptr).method();
            black_box(count);
        }
    }
}

fn benchmark_extend_pointer(c: &mut Criterion) {
    c.bench_function("extend_pointer", |b| {
        b.iter(|| {
            pointer();
        })
    });
}

criterion_group!(benches, benchmark_extend_pointer);
criterion_main!(benches);
