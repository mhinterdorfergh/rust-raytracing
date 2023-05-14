use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use crate::hittable::HittableList;

pub trait Loader {
    fn load_file(&self, file_name: &str) -> HittableList;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
