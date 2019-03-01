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

pub fn get_levenshtein_distance(first: &str, second: &str) -> usize {
    let row_length = first.chars().count();
    let column_length = second.chars().count();

    let first_vec: Vec<char> = first.chars().collect();
    let second_vec: Vec<char> = second.chars().collect();

    let mut matrix: Vec<Vec<usize>> = vec![vec![0; row_length + 1]; column_length + 1];

    let insert_cost = get_insert_cost();
    let delete_cost = get_delete_cost();

    println!("Матрица расстояний:");

    for i in 0..=column_length {
        for j in 0..=row_length {
            // заполняем первую строку индексами от 0 до 'first_length'
            if i == 0 {
                matrix[i][j] = j;
            // заполняем первый столбец индексами от 0 до 'second_length'
            } else if j == 0 {
                matrix[i][j] = i;
            } else {
                let top_cell = matrix[i - 1][j];
                let left_cell = matrix[i][j - 1];
                let top_left_cell = matrix[i - 1][j - 1];

                let delete_distance = top_cell + delete_cost;
                let insert_distance = left_cell + insert_cost;

                let match_distance =
                    top_left_cell + get_match_cost(first_vec[j - 1], second_vec[i - 1]);

                matrix[i][j] = vec![match_distance, delete_distance, insert_distance]
                    .iter()
                    .fold(match_distance, |min, element| cmp::min(min, *element));
            }
        }

        println!("{:?}", matrix[i]);
    }

    matrix[column_length][row_length]
}
