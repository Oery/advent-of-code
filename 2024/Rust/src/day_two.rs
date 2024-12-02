use std::io::Read;

fn load_inputs() -> Vec<Vec<i32>> {
    let mut input = String::new();

    let mut file = std::fs::File::open("./inputs/day_two.txt").unwrap();
    file.read_to_string(&mut input).unwrap();

    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|str| str.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn is_report_safe(report: &Vec<i32>) -> bool {
    let mut previous_level = &-1;
    let is_increasing = report.first().unwrap() - report.get(1).unwrap() < 0;

    for level in report {
        // Skip first element
        if previous_level < &0 {
            previous_level = level;
            continue;
        }

        // Level stayed the same
        if level == previous_level {
            return false;
        }

        // Level should be increasing
        if is_increasing && previous_level - level > 0 {
            return false;
        }

        // Level should be decreasing
        if !is_increasing && previous_level - level < 0 {
            return false;
        }

        // Level diff is too high
        if previous_level.abs_diff(*level) > 3 {
            return false;
        }

        previous_level = level;
    }

    true
}

fn get_total_safe_reports(reports: Vec<Vec<i32>>) -> i32 {
    reports.iter().map(is_report_safe).filter(|x| *x).count() as i32
}

fn is_report_safe_bruteforce(report: &Vec<i32>) -> bool {
    (0..report.len())
        .map(|i| {
            let mut new_vec = report.to_owned();
            new_vec.remove(i);
            is_report_safe(&new_vec)
        })
        .any(|x| x)
}

fn get_total_safe_reports_dampener(reports: Vec<Vec<i32>>) -> i32 {
    reports
        .iter()
        .map(is_report_safe_bruteforce)
        .filter(|x| *x)
        .count() as i32
}

pub fn day_two() {
    // Day 2 - Part 1
    let reports = load_inputs();
    let total_safe_reports = get_total_safe_reports(reports);
    println!("There are {total_safe_reports} Safe Reports");

    // Day 2 - Part 2
    let reports = load_inputs();
    let total_safe_reports_dampener = get_total_safe_reports_dampener(reports);
    println!("There are {total_safe_reports_dampener} Safe Reports using the Problem Dampener");
}
