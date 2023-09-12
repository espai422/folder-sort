use std::env;

#[derive(Debug)]
pub struct Args {
    pub path: String,
}

pub fn parse_args() -> Args {
    let args: Vec<String> = env::args().collect();

    let path = args.get(1).unwrap_or(&String::from("~/Downloads/")).clone();

    Args { path }
}
