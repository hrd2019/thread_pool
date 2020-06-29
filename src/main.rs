use thread_pool::{parse_config, Threadpool, Work};
use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");

    let config = parse_config();
    println!("{:#?}", config);

    let tp = Threadpool::new(config.max_num as usize);

    let job = Box::new(Work::new(56));

    tp.sendTask(job);
}