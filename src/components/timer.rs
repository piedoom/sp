use std::{collections::HashMap, hash::Hash, time::Duration};

use amethyst::ecs::{Component, DenseVecStorage};
use serde::{Deserialize, Serialize};

/// A general purpose timer structure. Note that this timer is not a `Component`, as only one of the type can be attached to an entity.
/// Instead, we can use a wrapping component to handle N amount of timers.
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Timer {
    start_time: Duration,
    duration: Duration,
}

/// Information about the status of a timer
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TimerStatus {
    /// Timer has run to or over its duration
    Complete(Duration),
    /// Timer has not yet run to or over its duration
    Incomplete(Duration),
}

impl Timer {
    /// Check if the timer has completed. If it hasn't, return time remaining.
    pub fn check(&self, current_time: &Duration) -> TimerStatus {
        let end_time = self.start_time + self.duration;
        // Because `Duration` uses unsigned integers, we first need to determine if we are over or below our target time.
        if current_time >= &end_time {
            // the timer is complete! Normal subtraction works here.
            TimerStatus::Complete(*current_time - end_time)
        } else {
            // we need to reverse the order of our subtraction for this to work
            TimerStatus::Incomplete(end_time - *current_time)
        }
    }

    /// Check if the timer has completed. If it has, reset it to the provided time.
    pub fn check_and_reset(&mut self, current_time: &Duration) -> TimerStatus {
        let check = self.check(current_time);
        if let TimerStatus::Complete(_) = check {
            self.reset(current_time.clone());
        }
        check
    }

    /// Reset the initial start time to a duration
    pub fn reset(&mut self, new_time: Duration) {
        self.start_time = new_time;
    }

    pub fn new(start_millis: u64, duration_millis: u64) -> Timer {
        Timer {
            start_time: Duration::from_millis(start_millis),
            duration: Duration::from_millis(duration_millis),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate(s: u64, d: u64) -> Timer {
        Timer {
            start_time: Duration::from_secs(s),
            duration: Duration::from_secs(d),
        }
    }

    #[test]
    fn incomplete_timer() {
        // timer will be complete at 15 seconds duration
        let t = generate(5, 10);
        assert_eq!(
            t.check(&Duration::from_secs(10)),
            TimerStatus::Incomplete(Duration::from_secs(5))
        );
    }

    #[test]
    fn complete_timer() {
        let t = generate(5, 10);
        assert_eq!(
            t.check(&Duration::from_secs(20)),
            TimerStatus::Complete(Duration::from_secs(5))
        );
    }

    #[test]
    fn just_complete_timer() {
        let t = generate(5, 10);
        assert_eq!(
            t.check(&Duration::from_secs(15)),
            TimerStatus::Complete(Duration::from_secs(0))
        );
    }
}
