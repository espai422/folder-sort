pub mod config;
pub mod cli;
mod sorter;

use std::io;
use std::io::ErrorKind;

use std::path::Path;
pub use crate::config::Config;

pub fn start(config :Config) {
    config.path;
    let a = sorter::sort(config.path);
}


// fn sort_folder(path: &Path) -> io::Result<()>  {
//     if !path.is_dir() {
//         return Err(io::Error::new(ErrorKind::Other, "Bruh"));
//     }
//
//     Ok(())

// }
