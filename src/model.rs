use crate::HittableList;
use crate::color::Color;
use crate::material::{Lambertian, Material};
use crate::texture::ImageTexture;
use crate::triangle::Triangle;
use crate::vec3::Point3;
use std::sync::Arc;

pub fn load(filename: &str) -> HittableList {
    let options = tobj::LoadOptions {
        triangulate: true,
        single_index: true,
        ignore_points: true,
        ignore_lines: true,
        ..Default::default()
    };
    let (models, materials) = tobj::load_obj(filename, &options).expect("cannot load obj");
    let materials = materials.unwrap_or_default();
    let mut world = HittableList::default();
    let mut mats = Vec::new();
    for mat in &materials {
        mats.push(create_material(mat));
    }
    for model in models {
        let mesh = model.mesh;
        let positions = &mesh.positions;
        let indices = &mesh.indices;
        let texcoords = &mesh.texcoords;
        for face in indices.chunks(3) {
            let i0 = face[0] as usize;
            let i1 = face[1] as usize;
            let i2 = face[2] as usize;

            let v0 = Point3::new(
                positions[i0 * 3] as f64,
                positions[i0 * 3 + 1] as f64,
                positions[i0 * 3 + 2] as f64,
            );

            let v1 = Point3::new(
                positions[i1 * 3] as f64,
                positions[i1 * 3 + 1] as f64,
                positions[i1 * 3 + 2] as f64,
            );

            let v2 = Point3::new(
                positions[i2 * 3] as f64,
                positions[i2 * 3 + 1] as f64,
                positions[i2 * 3 + 2] as f64,
            );
            let uv0 = (texcoords[i0 * 2] as f64, texcoords[i0 * 2 + 1] as f64);
            let uv1 = (texcoords[i1 * 2] as f64, texcoords[i1 * 2 + 1] as f64);
            let uv2 = (texcoords[i2 * 2] as f64, texcoords[i2 * 2 + 1] as f64);
            let mat = if let Some(id) = mesh.material_id {
                mats[id].clone()
            } else {
                Arc::new(Lambertian::new_color(Color::new(0.1, 0.5, 0.7)))
            };
            world.add(Arc::new(Triangle::new(
                v0,
                v1,
                v2,
                uv0,
                uv1,
                uv2,
                mat,
            )));
        }
    }
    world
}

pub fn create_material(mat: &tobj::Material) -> Arc<dyn Material + Send + Sync> {
    if let Some(texture_path) = &mat.diffuse_texture {
        let tex = Arc::new(ImageTexture::new(texture_path));
        return Arc::new(Lambertian::new(tex));
    }
    let diffuse = mat.diffuse.unwrap_or([0.1, 0.5, 0.7]);
    Arc::new(Lambertian::new_color(Color::new(
        diffuse[0] as f64,
        diffuse[1] as f64,
        diffuse[2] as f64,
    )))
}
