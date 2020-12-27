use crate::physics::collider::AAQuader;
use crate::world::coordinates::{BlockPosition, BlockVector};
use bevy::math::Vec3;
use bevy::prelude::Transform;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Ray{
    origin: Vec3,
    //normalized
    direction: Vec3,
}

pub struct GridSnap {
    ray: Ray,
    current_block: BlockPosition,
}

impl Iterator for GridSnap {
    type Item = BlockPosition;

    fn next(&mut self) -> Option<Self::Item> {
        let collider = unsafe {
            AAQuader::unchecked(
                self.current_block.lower_corner(),
                self.current_block.higher_corner(),
            )
        };
        let hit_info = self.ray.hit_info(
            &collider
        ).unwrap_or_else(||unreachable!("the ray didnt found its next block!"));
        let current = self.current_block;
        self.current_block += hit_info.leaving_face_normal;
        Some(current)

    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RayHitInfo {
    start: f32,
    end: f32,
    leaving_face_normal: BlockVector,
}

impl Ray {
    pub fn from_global_transform(transform: &Transform) -> Self {
        Ray::new(
            transform.translation,
            -transform.rotation.mul_vec3(Vec3::unit_z())
        )
    }
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray{
            origin,
            direction: direction.normalize(),
        }
    }
    pub fn translate(&mut self, vec: Vec3) {
        self.origin += vec;
    }
    pub fn translate_to(&mut self, vec: Vec3) {
        self.origin = vec;
    }
    pub fn translated(&self, vec: Vec3) -> Ray {
        Ray {
            origin: self.origin + vec,
            direction: self.direction,
        }
    }
    pub fn translated_to(&mut self, vec: Vec3) -> Ray {
        Ray {
            origin: vec,
            direction: self.direction,
        }
    }
    pub fn hit_info(&self, other: &AAQuader) -> Option<RayHitInfo> {
        fn calc_hit_interval(origin: f32, direction: f32, start: f32, end: f32) -> (f32, f32, i64) {
            let start_dist = start - origin;
            let end_dist = end - origin;

            if direction != 0.0 {
                if end_dist * direction > start_dist * direction {
                    //positive orientation
                    (start_dist / direction, end_dist / direction, 1)
                } else {
                    //negative orientation
                    (end_dist / direction, start_dist / direction, -1)
                }
            } else if start_dist * end_dist < 0.0 { //distances have different sings and are both != 0
                (-f32::INFINITY, f32::INFINITY, 0)//Always possible
            } else {
                (f32::INFINITY, -f32::INFINITY, 0)//Never possible
            }
        }

        let (x_start, x_end, x_or) = calc_hit_interval(
            self.origin.x,
            self.direction.x,
            other.lower().x,
            other.higher().x
        );
        let (y_start, y_end, y_or) = calc_hit_interval(
            self.origin.y,
            self.direction.y,
            other.lower().y,
            other.higher().y
        );
        let (z_start, z_end, z_or) = calc_hit_interval(
            self.origin.z,
            self.direction.z,
            other.lower().z,
            other.higher().z
        );



        let start = x_start.max(y_start).max(z_start);
        let end = x_end.min(y_end).min(z_end);

        if start < end {
            let leaving_face_normal = if z_end < y_end && z_end < x_end {
                BlockVector::new(0, 0, z_or)
            } else if y_end < x_end && y_end < z_end {
                BlockVector::new(0, y_or, 0)
            } else {
                BlockVector::new(x_or, 0, 0)
            };

            Some(RayHitInfo{
                start,
                end,
                leaving_face_normal,
            })
        } else {
            None
        }
    }
    pub fn grid_snap(&self) -> GridSnap {
        //this protects the programm from crashing if the values of the origin are integers
        //TODO: fix this in GridSnap::next
        let ray = self.translated(self.direction / 2.0);
        GridSnap {
            ray,
            current_block: BlockPosition::from_vector(self.origin)
        }
    }
}