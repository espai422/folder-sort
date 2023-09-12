pub mod cli;
pub mod config;
mod sorter;

pub use crate::config::Config;

pub fn start(config: Config) {
    let result = sorter::sort(config.path);
    match result {
        Ok(_) => println!("\nSuccessfully Sorted"),
        Err(_) => println!("\nError while Sorting"),
    }
}
