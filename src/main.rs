mod edit_distance;

use std::io;
use edit_distance::*;

fn main() {
    loop {
        let mut first_string = String::new();
        let mut second_string = String::new();

        println!("Введите первую строку:");
        let _ = io::stdin().read_line(&mut first_string);
        
        println!("Введите вторую строку:");
        let _ = io::stdin().read_line(&mut second_string);

        println!(
            "{:?}",
            get_levenshtein_distance(&first_string, &second_string)
        );
    }
}
