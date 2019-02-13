use std::cmp;

#[allow(dead_code)]
fn get_levenstein_distance(first: &str, second: &str) -> usize {
    let removeCost = 1;
    let insertCost = 1;
    let replaceCost = 1;

    let firstLength = first.len();
    let secondLength = second.len();

    let maxDistance = cmp::max(firstLength, secondLength);
    maxDistance
}


fn main() {
    let firstString = "toggle";
    let secondString = "google";

    println!("{}", get_levenstein_distance(firstString, secondString));
}
