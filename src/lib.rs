use serde_derive::Deserialize;
use std::fs::File;
use std::io::Read;

#[derive(Deserialize, Debug)]
pub struct Config {
    max_num: u64,   //最大线程数
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
