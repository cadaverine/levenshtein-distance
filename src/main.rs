use std::cmp;

#[allow(dead_code)]
fn get_match_cost(a: char, b: char) -> usize {
    if a == b {
        return 0;
    }
    return 1;
}

#[allow(dead_code)]
fn get_insert_cost() -> usize {
    return 1;
}

#[allow(dead_code)]
fn get_delete_cost() -> usize {
    return 1;
}



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
