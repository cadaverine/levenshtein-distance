mod edit_distance;
use edit_distance::*;

fn main() {
    let first_string = "овал";
    let second_string = "кровля";

    println!(
        "{:?}",
        get_levenshtein_distance(first_string, second_string)
    );
}
