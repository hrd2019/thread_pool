use thread_pool::parse_config;

fn main() {
    println!("Hello, world!");

    let config = parse_config();
    println!("{:#?}", config);
}
