use crate::HittableList;
use crate::material::Material;
use crate::triangle::Triangle;
use crate::vec3::Point3;
use std::sync::Arc;

pub fn load(filename: &str, mat: Arc<dyn Material + Send + Sync>) -> HittableList {
    let options = tobj::LoadOptions {
        triangulate: true,
        single_index: true,
        ..Default::default()
    };
    let (models, _materials) = tobj::load_obj(filename, &options).expect("cannot load obj");
    let mut world = HittableList::default();
    for model in models {
        let mesh = model.mesh;
        let positions = &mesh.positions;
        let indices = &mesh.indices;

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

            world.add(Arc::new(Triangle::new(v0, v1, v2, mat.clone())));
        }
    }
    world
}
