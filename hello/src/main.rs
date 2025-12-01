use std::thread;

fn main() {
    hello::run();
    let v = vec![1, 2, 3];
    for i in v {
        println!("{}", i);
    }

    let box_str = Box::new(String::from("world"));
    hello(&box_str);

    thread::spawn(|| {
        println!("hello, world!");
    })
    .join()
    .unwrap();
}

fn hello(str: &str) {
    println!("hello, {}!", str);
}
