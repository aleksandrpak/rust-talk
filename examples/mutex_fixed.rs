use std::thread;
use std::sync::{Mutex, Arc};
use std::time::Duration;

#[derive(Debug)]
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
    let foo = Arc::new(Mutex::new(Foo::new()));

    for _ in 0..10 {
        let reference = foo.clone();

        thread::spawn(move || {
            let mut foo = reference.lock().unwrap();
            foo.increment()
        });
    }

    thread::sleep(Duration::from_millis(50));

    println!("{:?}", foo);
}
