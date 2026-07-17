use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use std::cmp::Ordering;
use std::sync::Arc;

pub struct BVHNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: Aabb,
}

impl BVHNode {
    pub fn from_list(objects: &mut Vec<Arc<dyn Hittable>>) -> Self {
        BVHNode::new(objects, 0, objects.len())
    }
    pub fn new(objects: &mut Vec<Arc<dyn Hittable>>, start: usize, end: usize) -> Self {
        let mut bbox = Aabb::empty();
        #[allow(clippy::needless_range_loop)]
        for object_index in start..end {
            bbox = Aabb::from_boxes(&bbox, &objects[object_index].bounding_box());
        }
        let axis = bbox.longest_axis();
        let object_span = end - start;
        let (left, right): (Arc<dyn Hittable>, Arc<dyn Hittable>);

        if object_span == 1 {
            left = objects[start].clone();
            right = objects[start].clone();
        } else if object_span == 2 {
            left = objects[start].clone();
            right = objects[start + 1].clone();
        } else {
            match axis {
                0 => {
                    objects[start..end].sort_by(BVHNode::box_x_compare);
                }
                1 => {
                    objects[start..end].sort_by(BVHNode::box_y_compare);
                }
                _ => {
                    objects[start..end].sort_by(BVHNode::box_z_compare);
                }
            }
            let mid = start + object_span / 2;
            left = Arc::new(BVHNode::new(objects, start, mid));
            right = Arc::new(BVHNode::new(objects, mid, end));
        }
        Self { left, right, bbox }
    }

    fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis_index: usize) -> Ordering {
        let a_box = a.bounding_box();
        let b_box = b.bounding_box();
        let a_axis_interval = a_box.axis_interval(axis_index);
        let b_axis_interval = b_box.axis_interval(axis_index);
        a_axis_interval
            .min
            .partial_cmp(&b_axis_interval.min)
            .unwrap_or(Ordering::Equal)
    }
    fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        BVHNode::box_compare(a, b, 0)
    }
    fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        BVHNode::box_compare(a, b, 1)
    }
    fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        BVHNode::box_compare(a, b, 2)
    }
}
impl Hittable for BVHNode {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        if !self.bbox.hit(r, ray_t) {
            return false;
        }
        let hit_left = self.left.hit(r, ray_t, rec);
        let new_interval = Interval::new(ray_t.min, if hit_left { rec.t } else { ray_t.max });
        let hit_right = self.right.hit(r, new_interval, rec);
        hit_left || hit_right
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
