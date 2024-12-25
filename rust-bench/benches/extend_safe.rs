use criterion::{black_box, criterion_group, criterion_main, Criterion};

trait GetCount {
    fn get_count(&mut self) -> u32;
}

struct Node {
    count: u32,
}
impl Node {
    fn new(count: u32) -> Self {
        Node { count }
    }
}
impl GetCount for Node {
    fn get_count(&mut self) -> u32 {
        self.count += 1;
        self.count
    }
}

struct Element {
    node: Node,
}
impl Element {
    fn new(count: u32) -> Self {
        Element {
            node: Node::new(count),
        }
    }
}

struct DiveElement {
    element: Element,
}
impl DiveElement {
    fn new(count: u32) -> Self {
        DiveElement {
            element: Element::new(count),
        }
    }
}
impl GetCount for DiveElement {
    fn get_count(&mut self) -> u32 {
        self.element.node.get_count()
    }
}

fn vtable() {
    let mut dive_element = DiveElement::new(1);

    for _i in 0..10000 {
        let count = dive_element.get_count();
    black_box(count);
    }
}

fn benchmark_extend_vtable(c: &mut Criterion) {
    c.bench_function("extend_vtable", |b| {
        b.iter(|| {
            vtable();
        })
    });
}

criterion_group!(benches, benchmark_extend_vtable);
criterion_main!(benches);
