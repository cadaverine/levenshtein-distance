use std::cmp;

fn get_match_cost(a: char, b: char) -> usize {
    if a == b {
        return 0;
    }
    return 1;
}

fn get_insert_cost() -> usize {
    return 1;
}

fn get_delete_cost() -> usize {
    return 1;
}

fn get_levenstein_distance(first: &str, second: &str) -> usize {
    let first_length = first.len();
    let second_length = second.len();

    let first_vec: Vec<char> = first.chars().collect();
    let second_vec: Vec<char> = second.chars().collect();

    let mut matrix: Vec<Vec<usize>> = vec![vec![0; first_length + 1]; second_length + 1];

    let insert_cost = get_insert_cost();
    let delete_cost = get_delete_cost();

    for (i, row) in matrix.iter_mut().enumerate() {
        for (j, item) in row.iter_mut().enumerate() {
            // заполняем первую строку индексами от 0 до 'first_length'
            if i == 0 {
                *item = j;
            // заполняем первый столбец индексами от 0 до 'second_length'
            } else if j == 0 {
                *item = i;
            } else {
                let top_cell = matrix[i - 1][j];
                let left_cell = row[j - 1];
                let top_left_cell = matrix[i - 1][j - 1];

                let delete_distance = top_cell + delete_cost;
                let insert_distance = left_cell + insert_cost;
                let match_distance =
                    top_left_cell + get_match_cost(first_vec[j - 1], second_vec[i - 1]);

                *item = vec![match_distance, delete_distance, insert_distance]
                    .iter()
                    .fold(0, |min, element| cmp::min(min, *element));
            }
        }
    }

    matrix[first_length - 1][second_length - 1]
}

fn main() {
    let first_string = "toggle";
    let second_string = "google";

    println!("{}", get_levenstein_distance(first_string, second_string));
}
