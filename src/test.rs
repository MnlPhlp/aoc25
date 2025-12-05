use std::time::Duration;

use aoc23::{calc_day, Task};

fn test_day(day: usize, exp1: &str, exp2: &str) {
    let mut res1 = String::new();
    let mut res2 = String::new();
    let mut duration = Duration::default();
    calc_day(
        day,
        &mut res1,
        &mut res2,
        &mut duration,
        false,
        Task::Both,
        false,
    );
    assert_eq!(exp1, res1, "task 1 gave wrong result");
    assert_eq!(exp2, res2, "task 2 gave wrong result");
}

#[test]
pub fn day1() {
    test_day(1, "1100", "6358");
}

#[test]
pub fn day2() {
    test_day(2, "64215794229", "85513235135");
}

#[test]
pub fn day3() {
    test_day(3, "16812", "166345822896410");
}

#[test]
pub fn day4() {
    test_day(4, "1419", "8739");
}

#[test]
pub fn day5() {
    test_day(5, "737", "357485433193284");
}

#[test]
pub fn day6() {
    test_day(6, "Not implemented", "Not implemented");
}

#[test]
pub fn day7() {
    test_day(7, "Not implemented", "Not implemented");
}

#[test]
pub fn day8() {
    test_day(8, "Not implemented", "Not implemented");
}

#[test]
pub fn day9() {
    test_day(9, "Not implemented", "Not implemented");
}

#[test]
pub fn day10() {
    test_day(10, "Not implemented", "Not implemented");
}

#[test]
pub fn day11() {
    test_day(11, "Not implemented", "Not implemented");
}

#[test]
pub fn day12() {
    test_day(12, "Not implemented", "Not implemented");
}
