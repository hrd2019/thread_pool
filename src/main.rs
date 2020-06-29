use thread_pool::{parse_config, Threadpool, Work};
use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");

    let config = parse_config();
    println!("{:#?}", config);

    let tp = Threadpool::new(config.max_num as usize);

    thread::spawn(move || {
        for id in 1..400{
            let job = Box::new(Work::new(id));
            tp.send_task(job);

            // thread::sleep(Duration::from_secs(1));
        }
    });

    loop{
        println!("----");
        thread::sleep(Duration::from_secs(5));
    }
}