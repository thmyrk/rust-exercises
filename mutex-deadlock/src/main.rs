use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let arc_1 = Arc::new(Mutex::new(0));
    let arc_2 = Arc::new(Mutex::new("text"));

    let mut handles = vec![];

    let counter_1 = Arc::clone(&arc_1);
    let counter_2 = Arc::clone(&arc_1);
    let text_1 = Arc::clone(&arc_2);
    let text_2 = Arc::clone(&arc_2);

    handles.push({
        thread::spawn(move || {
            let value = counter_1.lock().unwrap();

            thread::sleep(Duration::from_secs(1));
            println!("{value}");

            // this thread maintains lock of counter and wants to lock text
            println!("{}", text_1.lock().unwrap());
        })
    });

    handles.push({
        thread::spawn(move || {
            let value = text_2.lock().unwrap();

            thread::sleep(Duration::from_secs(1));

            println!("{value}");

            // this thread maintains lock of text and wants to lock counter
            println!("{}", counter_2.lock().unwrap());
        })
    });

    for handle in handles {
        handle.join().unwrap()
    }
}
