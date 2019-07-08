/*
 * Rustの高度な機能（関数、クロージャ）。
 * CreatedAt: 2019-07-08
 */
fn main() {
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        .map(|i| i.to_string())
//        .map(ToString::to_string)
        .collect();
}


