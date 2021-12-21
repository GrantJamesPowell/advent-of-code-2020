#![allow(dead_code)]
#![feature(int_abs_diff)]
#![feature(bool_to_option)]

use std::ops::RangeInclusive;
use std::cmp::Ordering::*;
use itertools::unfold;

fn main() {}

fn hits_target(
    start_vel: isize,
    target: &RangeInclusive<isize>,
    accel: impl Fn(isize) -> isize,
) -> bool {
    // Generate the positions at each step
    path_from_origin(start_vel, accel)
    // Find the distance to the target
    .map(|pos| (pos, distance_to_target(pos, target)))
    // Keep taking them while the distance to target is decreasing
    .scan(
        usize::MAX,
        |previous_distance, (pos, distance_to_target)| {
            let is_getting_closer = *previous_distance > distance_to_target;
            *previous_distance = distance_to_target;
            is_getting_closer.then_some(pos)
        },
    ).any(|pos| target.contains(&pos))
}

fn path_from_origin(start_vel: isize, accel: impl Fn(isize) -> isize) -> impl Iterator<Item = isize> { 
    unfold((0, start_vel), move |(pos, vel)| {
        *pos = *pos + *vel;
        *vel = accel(*vel);

        Some(*pos)
    })
}

fn distance_to_target(pos: isize, target: &RangeInclusive<isize>) -> usize {
    let from_start = pos.abs_diff(*target.start());
    let from_end = pos.abs_diff(*target.end());
    if target.contains(&pos) {
        0
    } else {
        from_start.min(from_end)
    }
}

fn next_velocity_towards_zero(vel: isize) -> isize {
    match (vel).cmp(&0) {
        Greater => vel - 1,
        Less => vel + 1,
        Equal => vel,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_velocity_towards_zero() {
        assert_eq!(next_velocity_towards_zero(10), 9);
        assert_eq!(next_velocity_towards_zero(-10), -9);
        assert_eq!(next_velocity_towards_zero(0), 0);
    }

    #[test]
    fn test_distance_to_target() {
        assert_eq!(distance_to_target(10, &(10..=20)), 0);
        assert_eq!(distance_to_target(2, &(10..=20)), 8);
        assert_eq!(distance_to_target(25, &(10..=20)), 5);
    }

    #[test]
    fn test_hits_target() {
        // Simple case
        assert!(hits_target(1, &(10..=20), |x| x));
        // Over steps
        assert!(!hits_target(100, &(10..=20), |x| x));
        // Never reaches
        assert!(!hits_target(2, &(10..=20), |x| x - 1));
        // From example
        assert!(hits_target(7, &(20..=30), next_velocity_towards_zero));
        assert!(hits_target(2, &(-10..=5), |x| x - 1));
    }
}
