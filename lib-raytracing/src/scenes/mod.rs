use std::{
    fmt::Debug,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

mod mtlloader;
mod objloader;
use crate::{hittable::HittableList, material, materials};

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn load_scene<P>(obj_directory: P) -> HittableList
where
    P: AsRef<Path> + Debug,
{
    let (models, materials) =
        tobj::load_obj(&obj_directory, &tobj::GPU_LOAD_OPTIONS).expect("Failed loading scene");

    let materials = match materials {
        Ok(materials) => mtlloader::load_materials(materials),
        Err(_) => Vec::new(),
    };
    objloader::load_objects(&materials, &models)
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
