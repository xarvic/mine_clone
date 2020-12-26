use bevy::prelude::{Vec3, Transform};
use crate::world::coordinates::{BlockVector, BlockPosition};

pub trait Intersection<Rhs> {
    fn intersect(&self, other: &Rhs) -> bool;
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Quader {
    lower: Vec3,
    higher: Vec3,
}

impl Quader {
    pub fn new(lower: Vec3, higher: Vec3) -> Self {
        assert!(lower.x <= higher.x && lower.y <= higher.y && lower.z <= higher.z, "invalid collider parameters!");
        unsafe {
            Self::unchecked(lower, higher)
        }
    }
    pub unsafe fn unchecked(lower: Vec3, higher: Vec3) -> Self {
        Self {
            lower,
            higher,
        }
    }
    pub fn center_size(center: Vec3, size: Vec3) -> Self {
        let half_size = size.abs() / 2.0;
        unsafe {
            Self::unchecked(center - half_size, center + half_size)
        }
    }
    pub fn size(&self) -> Vec3 {
        self.higher - self.lower
    }
    pub fn lower(&self) -> Vec3 {
        self.lower
    }
    pub fn higher(&self) -> Vec3 {
        self.higher
    }
    pub fn translate(&mut self, translation: Vec3) {
        self.lower += translation;
        self.higher += translation;
    }
    pub fn scale(&mut self, factor: f32) {
        let half_size = self.size() / 2.0;
        let center = self.lower + half_size;

        self.lower = center - half_size * factor;
        self.higher = center + half_size * factor;
    }
    pub fn volume(&self) -> f32 {
        let size = self.size();
        size.x * size.y * size.z
    }
}

impl Intersection<Quader> for Quader {
    fn intersect(&self, other: &Self) -> bool {
        let max_dist = self.size() + other.size();

        (self.higher.x - other.lower.x < max_dist.x || other.higher.x - self.lower.x < max_dist.x) &&
            (self.higher.y - other.lower.y < max_dist.y || other.higher.y - self.lower.y < max_dist.y) &&
            (self.higher.z - other.lower.z < max_dist.z || other.higher.z - self.lower.z < max_dist.z)
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Ray{
    origin: Vec3,
    //normalized
    direction: Vec3,
}

pub trait BlockIter: Iterator<Item=BlockPosition> {
    fn last_which<F>()
}

pub struct GridSnap {
    ray: Ray,
    current_block: BlockPosition,
}

impl Iterator for GridSnap {
    type Item = BlockPosition;

    fn next(&mut self) -> Option<Self::Item> {
        let collider = unsafe {
            Quader::unchecked(
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
    pub fn hit_info(&self, other: &Quader) -> Option<RayHitInfo> {
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
            other.lower.x,
            other.higher.x
        );
        let (y_start, y_end, y_or) = calc_hit_interval(
            self.origin.y,
            self.direction.y,
            other.lower.y,
            other.higher.y
        );
        let (z_start, z_end, z_or) = calc_hit_interval(
            self.origin.z,
            self.direction.z,
            other.lower.z,
            other.higher.z
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

impl Intersection<Quader> for Ray {
    fn intersect(&self, other: &Quader) -> bool {
        self.hit_info(other).is_some()
    }
}