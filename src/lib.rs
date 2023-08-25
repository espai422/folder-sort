pub mod config;
pub mod cli;
mod sorter;

pub use crate::config::Config;

pub fn start(config :Config) {
    config.path;
    let result = sorter::sort(config.path);
    match  result {
        Ok(_) => println!("\nSuccessfully Sorted"),
        Err(_) => println!("\nError while Sorting")
    }
}
