use thread_pool::{parse_config, build_thread};
use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");

    let config = parse_config();
    println!("{:#?}", config);

    unsafe {
        let h = build_thread(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        // h.join().unwrap();
    }


}