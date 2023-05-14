use crate::{
    hittable::HittableList, materials::lambertian::Lambertian, objects::triangle::Triangle,
    vec3::Vec3,
};

use super::loader::{read_lines, Loader};

pub struct OBJLoader {}

impl Loader for OBJLoader {
    fn load_file(&self, file_name: &str) -> crate::hittable::HittableList {
        let mut vertices: Vec<Vec3> = Vec::new();
        let mut scene: HittableList = Default::default();
        if let Ok(lines) = read_lines(file_name) {
            for line in lines {
                if let Ok(entry) = line {
                    let parts: Vec<&str> = entry.split(" ").collect();
                    match parts[0] {
                        "v" => parse_vec(&parts, &mut vertices),
                        "f" => parse_face(&parts, &vertices, &mut scene),
                        _ => continue,
                    }
                }
            }
        } else {
            log::error!("Could not find file");
        }
        scene
    }
}

fn parse_face(parts: &[&str], vertices: &[Vec3], scene: &mut HittableList) {
    let indices: Vec<usize> = parts
        .into_iter()
        .skip(1)
        .map(|p| if let Ok(value) = p.parse() { value } else { 0 })
        .collect();
    let root = vertices[indices[0]];
    indices.windows(2).skip(1).for_each(|window| {
        scene.add(Triangle::new(
            root,
            vertices[window[0] - 1],
            vertices[window[1] - 1],
            Lambertian {
                color: Vec3::new(0.0, 1.0, 0.0),
            },
        ))
    })
}

fn parse_vec(parts: &[&str], vertices: &mut Vec<Vec3>) {
    if parts.len() != 4 {
        log::info!("skipping vertex");
    }
    let vec = Vec3::new(
        parts[1].parse().expect("failed parsing"),
        parts[2].parse().expect("failed parsing"),
        parts[3].parse().expect("failed parsing"),
    );
    vertices.push(vec);
}

