use serde_derive::Deserialize;
use std::fs::File;
use std::io::Read;
use std::thread;
use std::sync::{Arc, mpsc, Mutex};
use std::time::Duration;

pub struct Threadpool{
    workers:Vec<Worker>,
    sender:mpsc::Sender<Job>,
}

impl Threadpool{
    pub fn new(size:usize) -> Threadpool{
        let (sender,receiver) = mpsc::channel();
        let mut workers = Vec::with_capacity(size);

        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size{
            workers.push(Worker::new(id, Arc::clone(&receiver)));

        }

        Threadpool{
            workers,
            sender,
        }
    }

    pub fn send_task(&self, j: Job){
        self.sender.send(j).unwrap();
    }
}

impl Drop for Threadpool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

pub struct Work{
    id:usize,
}

pub trait Process{
    fn exec(&self);
}

type Job = Box<dyn Process + Send + 'static>;

impl Work{
    pub fn new(id:usize) -> Work{
        Work{
            id,
        }
    }
}

impl Process for Work{
    fn exec(&self){
        println!("---->{}", self.id);
    }
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub max_num: u64,   //最大线程数
    min_num: u64,   //最小线程数
    idle_num: u64,  //空闲线程数
    idle_time: u64, //空闲时长(秒)
}

pub fn parse_config() -> Config {
    let file_path = "config/config.toml";
    let mut file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => panic!("open file {} failed.\n err:{}\n", file_path, e),
    };

    let mut str_buffer = String::new();
    match file.read_to_string(&mut str_buffer) {
        Ok(s) => s,
        Err(e) => panic!("read file failed: {}", e),
    };

    let config: Config = toml::from_str(&str_buffer).unwrap();
    config
}

pub struct Worker{
    id:usize,
    thread:Option<thread::JoinHandle<()>>
}

impl Worker{
    pub fn new(id:usize, rec:Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker{
        let thread = thread::spawn(move|| {
            loop {
                if let Ok(job) = rec.lock().unwrap().recv() {
                    println!("thread[{}] got a job", id);
                    job.exec();
                }

                thread::sleep(Duration::from_micros(200));
            }
        });

        Worker{
            id,
            thread:Some(thread),
        }
    }
}
