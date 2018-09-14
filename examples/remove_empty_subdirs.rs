#[macro_use]
extern crate clap;

extern crate remove_empty_subdirs;

use std::path::Path;

use clap::{App, Arg};

use remove_empty_subdirs::remove_empty_subdirs;

fn main() {
    let matches = App::new("remove_empty_subdirs")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Remove empty directories under a directory.")
        .arg(
            Arg::with_name("dir")
                .help("Directory to deal with")
                .index(1)
                .required(true),
        ).get_matches();

    let dir = matches.value_of("dir").unwrap();
    let path = Path::new(dir);
    match remove_empty_subdirs(path) {
        Ok(()) => {}
        Err(err) => println!("{:?}", err.to_string()),
    }
}
