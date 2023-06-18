use std::sync::Arc;

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use crate::materials::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal};

pub fn load_materials(
    materials: Vec<tobj::Material>,
) -> Vec<Option<Arc<dyn crate::material::Material>>> {
    materials
        .par_iter()
        .map(|material| extract_material(&material))
        .collect()
}

fn extract_material(material: &tobj::Material) -> Option<Arc<dyn crate::material::Material>> {
    if let Some(index_of_refraction) = material.optical_density {
        Some(Arc::new(Dielectric {
            index_of_refraction: index_of_refraction.into(),
        }))
    } else if let (Some(ambient_color), Some(diffuse_color)) = (material.ambient, material.diffuse)
    {
        Some(Arc::new(Lambertian {
            color: crate::vec3::Vec3 {
                x: diffuse_color[0].into(),
                y: diffuse_color[1].into(),
                z: diffuse_color[2].into(),
            } * crate::vec3::Vec3 {
                x: ambient_color[0].into(),
                y: ambient_color[1].into(),
                z: ambient_color[2].into(),
            },
        }))
    } else if let (Some(ambient), Some(shininess)) = (material.ambient, material.shininess) {
        Some(Arc::new(Metal {
            color: crate::vec3::Vec3 {
                x: ambient[0].into(),
                y: ambient[1].into(),
                z: ambient[2].into(),
            },
            fuzz: shininess.into(),
        }))
    } else {
        None
    }
}
