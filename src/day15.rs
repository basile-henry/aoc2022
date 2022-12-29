use alloc::vec::Vec;
use core::alloc::Allocator;
use core::cmp::Ordering;
use core::fmt::Debug;

use crate::hash::HashSet;
use crate::hash_set;

use nom::bytes::complete::tag;
use nom::character::complete::*;
use nom::sequence::*;

#[cfg_attr(feature = "trace", tracing::instrument(skip_all))]
pub fn day15<A: Allocator + Debug + Copy>(alloc: A, input: &str) -> (usize, usize) {
    let (sensors, beacons) = parse(alloc, input);

    let part1 = positions_not_present(alloc, 2000000, &sensors, &beacons);
    let beacon = find_isolated(alloc, 4000000, 4000000, &sensors).unwrap();

    let part2 = beacon.x as usize * 4000000 + beacon.y as usize;

    (part1, part2)
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Pos {
    x: i32,
    y: i32,
}

fn parse<A: Allocator + Copy>(alloc: A, input: &str) -> (Vec<(Pos, u32), A>, HashSet<Pos, A>) {
    let mut sensors = Vec::with_capacity_in(16, alloc);
    let mut beacons = hash_set!(16, alloc);

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let (_, sx, _, sy, _, bx, _, by) = tuple((
            tag::<_, _, ()>("Sensor at x="),
            i32,
            tag(", y="),
            i32,
            tag(": closest beacon is at x="),
            i32,
            tag(", y="),
            i32,
        ))(line)
        .unwrap()
        .1;

        let manhattan = sx.abs_diff(bx) + sy.abs_diff(by);

        sensors.push((Pos { x: sx, y: sy }, manhattan));
        beacons.insert(Pos { x: bx, y: by });
    }

    (sensors, beacons)
}

fn sensor_no_beacon_interval_on(on_y: i32, sensor: (Pos, u32)) -> Option<Interval> {
    let y_dist = sensor.0.y.abs_diff(on_y);

    match sensor.1.cmp(&y_dist) {
        Ordering::Less => None,
        Ordering::Equal => Some(Interval::new(sensor.0.x, sensor.0.x)),
        Ordering::Greater => {
            let extra = (sensor.1 - y_dist) as i32;
            Some(Interval::new(sensor.0.x - extra, sensor.0.x + extra))
        }
    }
}

fn positions_not_present<A: Allocator + Debug + Clone>(
    alloc: A,
    on_y: i32,
    sensors: &[(Pos, u32)],
    beacons: &HashSet<Pos, A>,
) -> usize {
    let mut interval_set = IntervalSet::new_in(alloc);

    for sensor in sensors.iter() {
        if let Some(interval) = sensor_no_beacon_interval_on(on_y, *sensor) {
            interval_set.insert(interval);
        }
    }

    let beacon_in_interval = beacons
        .iter()
        .filter(|pos| pos.y == on_y && interval_set.contains(&pos.x))
        .count();

    interval_set.count() - beacon_in_interval
}

fn find_isolated<A: Allocator + Debug + Copy>(
    alloc: A,
    up_to_x: i32,
    up_to_y: i32,
    sensors: &[(Pos, u32)],
) -> Option<Pos> {
    let mut interval_set = IntervalSet::new_in(alloc);

    for y in 0..=up_to_y {
        for sensor in sensors.iter() {
            if let Some(interval) = sensor_no_beacon_interval_on(y, *sensor) {
                interval_set.insert(interval);
            }
        }

        interval_set.limit_by(Interval::new(0, up_to_x));

        if interval_set.intervals.len() > 1 {
            assert_eq!(interval_set.intervals.len(), 2);
            let x = interval_set.intervals[0].b + 1;

            return Some(Pos { x, y });
        }

        interval_set.intervals.clear();
    }

    None
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Interval {
    // invariant: a <= b
    a: i32,
    b: i32,
}

impl Interval {
    fn new(a: i32, b: i32) -> Self {
        Self {
            a: a.min(b),
            b: a.max(b),
        }
    }

    fn join(&self, other: &Self) -> Option<Self> {
        let combined = Self {
            a: self.a.min(other.a),
            b: self.b.max(other.b),
        };

        match self.a.cmp(&other.a) {
            Ordering::Less if self.b < other.a => None,
            Ordering::Greater if self.a > other.b => None,
            _ => Some(combined),
        }
    }

    fn count(&self) -> usize {
        (self.b - self.a + 1) as usize
    }

    fn contains(&self, x: &i32) -> bool {
        (self.a..=self.b).contains(x)
    }
}

#[derive(Debug)]
struct IntervalSet<A: Allocator> {
    // invariant: ordered, non-overlapping
    intervals: Vec<Interval, A>,
}

impl<A: Allocator> IntervalSet<A> {
    fn new_in(alloc: A) -> IntervalSet<A> {
        IntervalSet {
            intervals: Vec::new_in(alloc),
        }
    }

    fn limit_by(&mut self, limit: Interval) {
        let mut start_remove_count = 0;
        let mut end_remove_count = 0;

        for i in 0..self.intervals.len() {
            if limit.a > self.intervals[i].a {
                if limit.a > self.intervals[i].b {
                    start_remove_count += 1;
                } else {
                    self.intervals[i].a = limit.a;
                }
            }

            if limit.b < self.intervals[i].b {
                if limit.b < self.intervals[i].a {
                    end_remove_count += 1;
                } else {
                    self.intervals[i].b = limit.b;
                }
            }
        }

        self.intervals.splice(0..start_remove_count, []);
        self.intervals
            .splice((self.intervals.len() - end_remove_count).., []);
    }

    fn insert(&mut self, mut interval: Interval) {
        let start_ix = self
            .intervals
            .binary_search_by(|x| x.a.cmp(&interval.a))
            .unwrap_or_else(|x| x);

        let mut low_ix = None;
        let mut high_ix = None;

        let ix = start_ix;

        if let Some(new_interval) = self.intervals.get(ix).and_then(|x| interval.join(x)) {
            interval = new_interval;
            low_ix = Some(ix);
        }

        if let Some(mut ix) = start_ix.checked_sub(1) {
            while let Some(new_interval) = self.intervals.get(ix).and_then(|x| interval.join(x)) {
                interval = new_interval;
                low_ix = Some(ix);

                if ix == 0 {
                    break;
                }

                ix -= 1;
            }
        }

        let mut ix = start_ix;

        while let Some(new_interval) = self.intervals.get(ix).and_then(|x| interval.join(x)) {
            interval = new_interval;
            high_ix = Some(ix);
            ix += 1;
        }

        match (low_ix, high_ix) {
            (Some(low_ix), Some(high_ix)) => {
                let new_interval = [interval];
                self.intervals.splice(low_ix..=high_ix, new_interval);
            }
            (Some(low_ix), None) => {
                self.intervals[low_ix] = interval;
            }
            (None, Some(high_ix)) => {
                self.intervals[high_ix] = interval;
            }
            (None, None) => self.intervals.insert(start_ix, interval),
        }
    }

    fn count(&self) -> usize {
        self.intervals.iter().map(|i| i.count()).sum()
    }

    fn contains(&self, x: &i32) -> bool {
        self.intervals.iter().any(|i| i.contains(x))
    }
}

#[test]
fn intervals() {
    let bump = bumpalo::Bump::new();
    let mut set = IntervalSet::new_in(&bump);

    set.insert(Interval::new(1, 3));
    assert_eq!(&set.intervals[..], &[Interval::new(1, 3)]);

    set.insert(Interval::new(4, 4));
    assert_eq!(
        &set.intervals[..],
        &[Interval::new(1, 3), Interval::new(4, 4)]
    );

    set.insert(Interval::new(-3, 0));
    assert_eq!(
        &set.intervals[..],
        &[
            Interval::new(-3, 0),
            Interval::new(1, 3),
            Interval::new(4, 4)
        ]
    );

    set.insert(Interval::new(1, 2));
    assert_eq!(
        &set.intervals[..],
        &[
            Interval::new(-3, 0),
            Interval::new(1, 3),
            Interval::new(4, 4)
        ]
    );

    set.insert(Interval::new(-1, 2));
    assert_eq!(
        &set.intervals[..],
        &[Interval::new(-3, 3), Interval::new(4, 4)]
    );

    set.insert(Interval::new(-3, 10));
    assert_eq!(&set.intervals[..], &[Interval::new(-3, 10)]);

    set.insert(Interval::new(10, 15));
    assert_eq!(&set.intervals[..], &[Interval::new(-3, 15)]);
}

#[test]
fn both_parts() {
    let bump = bumpalo::Bump::new();
    let example = r#"
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
"#;

    let (sensors, beacons) = parse(&bump, example);

    assert_eq!(positions_not_present(&bump, 10, &sensors, &beacons), 26);
    assert_eq!(
        find_isolated(&bump, 20, 20, &sensors),
        Some(Pos { x: 14, y: 11 })
    );
}
