use bevy::math::Vec3;

struct Collider{
}

pub trait Intersection<Rhs> {
    fn intersect(&self, other: &Rhs) -> bool;
    fn impact_volume(&self, other: Rhs) -> Vec3;
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct AAQuader {
    lower: Vec3,
    higher: Vec3,
}

impl AAQuader {
    pub const fn new(lower: Vec3, higher: Vec3) -> Self {
        assert!(lower.x <= higher.x && lower.y <= higher.y && lower.z <= higher.z, "invalid collider parameters!");
        unsafe {
            Self::unchecked(lower, higher)
        }
    }
    pub const unsafe fn unchecked(lower: Vec3, higher: Vec3) -> Self {
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

    ///The signed overlapping volume of the quaders
    pub fn impact_volume(&self, other: AAQuader) -> Vec3 {
        #[inline(always)]
        fn value(l1: f32, h1: f32, l2: f32, h2: f32) -> f32 {
            let left_to_right = h2 - l1;
            let right_to_left = h1 - l2;
            //ensure that the return value is 0 if one of the values negative
            if left_to_right > 0.0 && right_to_left > 0.0 {
                if left_to_right < right_to_left {
                    left_to_right
                } else {
                    -right_to_left
                }
            } else {
                0.0
            }
        }

        Vec3::new(value(self.lower.x, self.higher.x, other.lower.x, other.higher.x),
                value(self.lower.y, self.higher.y, other.lower.y, other.higher.y),
                value(self.lower.z, self.higher.z, other.lower.z, other.higher.z))
    }
}

impl Intersection<AAQuader> for AAQuader {
    fn intersect(&self, other: &Self) -> bool {
        let impact = self.impact_volume(*other);
        impact.x != 0.0 && impact.x != 0.0 && impact.x != 0.0
    }

    fn impact_volume(&self, other: AAQuader) -> Vec3 {
        self.impact_volume(other)
    }
}