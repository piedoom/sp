use amethyst::{core::math::Vector3, ecs::prelude::*};

pub struct DistanceLimit {
    starting_position: Vector3<f32>,
    max_distance: f32,
}

impl DistanceLimit {
    /// Determine if a vector is out of the range of this distance limit.
    /// Returns true if the distance is within the valid limit.
    pub fn check(&self, pos: &Vector3<f32>) -> bool {
        // check the magnitude of the difference between the start and current vector
        let diff = pos - self.starting_position;
        diff.magnitude_squared() <= self.max_distance.powi(2)
    }

    pub fn new(starting_position: Vector3<f32>, max_distance: f32) -> Self {
        Self {
            starting_position,
            max_distance,
        }
    }
}

impl Component for DistanceLimit {
    type Storage = DenseVecStorage<Self>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        let d = DistanceLimit {
            starting_position: Vector3::zeros(),
            max_distance: 1f32,
        };
        let n1 = Vector3::new(1f32, 0f32, 0f32);
        let n2 = Vector3::new(2f32, 0f32, 0f32);
        assert_eq!(d.check(&n1), true);
        assert_eq!(d.check(&n2), false);
    }
}
