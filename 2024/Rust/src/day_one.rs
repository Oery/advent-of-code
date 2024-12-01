use crate::utils::load_inputs;

fn calculate_total_distance(mut list_one: Vec<u32>, mut list_two: Vec<u32>) -> u32 {
    list_one.sort();
    list_two.sort();

    list_one
        .iter()
        .zip(list_two.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum()
}

fn calculate_total_similarity(list_one: Vec<u32>, list_two: Vec<u32>) -> u32 {
    list_one
        .iter()
        .map(|a| a * list_two.iter().filter(|b| *b == a).count() as u32)
        .sum()
}

pub fn day_one() {
    // Day 1 - Part 1
    let (list_one, list_two) = load_inputs("day_one");
    let total = calculate_total_distance(list_one, list_two);
    println!("The Total Distance is {total}");

    // Day 1 - Part 2
    let (list_one, list_two) = load_inputs("day_one");
    let similarity = calculate_total_similarity(list_one, list_two);
    println!("The Total Similarity is {similarity}");
}
