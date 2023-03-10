extern crate spin;
use std::{sync::Arc, thread};

fn main() {
    let counter = Arc::new(spin::Mutex::new(0));

    let thread = thread::spawn({
        let counter = counter.clone();
        move || {
            for _ in 0..100 {
		*counter.lock() += 1;
		let cnt = *counter.lock();
        	println!("COUNTER = {} @parent",cnt);
            }
        }
    });

    for _ in 0..100 {
	*counter.lock() += 1;
	let  cnt = *counter.lock();
        println!("COUNTER = {} @sub",cnt);
    }

    thread.join().unwrap();

    assert_eq!(*counter.lock(), 200);
}
