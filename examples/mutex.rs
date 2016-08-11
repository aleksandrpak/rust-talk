use std::thread;

struct Foo {
    counter: usize,
}

impl Foo {
    pub fn new() -> Foo {
        Foo { counter: 0 }
    }

    pub fn increment(&mut self) {
        self.counter += 1
    }
}

pub fn main() {
    let mut foo = Foo::new();

    // for _ in 0..10 {
    thread::spawn(|| {
        foo.increment();
    });
    // }
}
