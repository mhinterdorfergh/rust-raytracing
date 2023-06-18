use std::sync::Arc;

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use tobj::Model;

use crate::{
    hittable::{Hittable, HittableList},
    material::Material,
    materials::lambertian::Lambertian,
    objects::triangle::Triangle,
    vec3::Vec3,
};

const FALLBACK_MATERIAL: Lambertian = Lambertian {
    color: Vec3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    },
};
pub fn load_objects(
    materials: &Vec<Option<Arc<dyn Material>>>,
    objects: &Vec<Model>,
) -> crate::hittable::HittableList {
    let mut scene: HittableList = Default::default();

    scene.objects = objects
        .par_iter()
        .map(|object| {
            let object = &object.mesh;
            if !object.face_arities.is_empty() {
                return Vec::new();
            }
            let vertices: Vec<Vec3> = object
                .positions
                .chunks(3)
                .map(|chunk| Vec3 {
                    x: chunk[0].into(),
                    y: chunk[1].into(),
                    z: chunk[2].into(),
                })
                .collect();
            let material = match object.material_id {
                Some(material_id) => match materials[material_id].clone() {
                    Some(material) => material,
                    None => {
                        log::debug!("Using fallback material");
                        Arc::new(FALLBACK_MATERIAL.clone())
                    }
                },
                None => {
                    log::debug!("Using fallback material");
                    Arc::new(FALLBACK_MATERIAL.clone())
                }
            };
            object
                .indices
                .chunks(3)
                .map(|chunk| {
                    let hittable: Box<dyn Hittable> = Box::new(Triangle {
                        a: vertices[chunk[0] as usize],
                        b: vertices[chunk[1] as usize],
                        c: vertices[chunk[2] as usize],
                        material: material.clone(),
                    });
                    hittable
                })
                .collect()
        })
        .flatten()
        .collect();
    scene
}
