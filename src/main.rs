mod cli;

use std::collections::HashMap;
use cli::parse_args;
use sort_download::{start, Config};
use std::path::Path;



fn main() {
    let args = parse_args();

    let config = Config {
        path: Path::new(&args.path),
    };

    start(config);
}
