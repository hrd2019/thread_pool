use thread_pool::{parse_config, build_thread, Threadpool, Work};
use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");

    let config = parse_config();
    println!("{:#?}", config);

    let tp = Threadpool::new(config.max_num as usize);

    let job = Box::new(Work::new(56));

    tp.sendTask(job);

    // unsafe {
    //     let h = build_thread(|| {
    //         for i in 1..10 {
    //             println!("hi number {} from the spawned thread!", i);
    //             thread::sleep(Duration::from_millis(1));
    //         }
    //     });
    //
    //     // h.join().unwrap();
    // }


}