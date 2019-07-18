use crate::components::Tracker;
use amethyst::core::{math::Vector3, Transform};
use amethyst::ecs::prelude::*;

#[derive(Default)]
pub struct TrackerSystem {
    changeset: ChangeSet<Vector3<f32>>,
}

impl<'a> System<'a> for TrackerSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Tracker>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, (entities, trackers, mut transforms): Self::SystemData) {
        // Thank you @kel!
        self.changeset.clear();
        for (entity, tracker, _) in (&entities, &trackers, &transforms).join() {
            // This is allowed because we're borrowing immutably
            if let Some(target_transform) = transforms.get(tracker.entity()) {
                // This is a value applied to the current entity in the join
                self.changeset.add(entity, *target_transform.translation());
            }
        }

        // Then applying our values is super simple!
        for (transform, value) in (&mut transforms, &self.changeset).join() {
            transform.set_translation_xyz(value.x, transform.translation().y, value.z);
        }
    }
}
