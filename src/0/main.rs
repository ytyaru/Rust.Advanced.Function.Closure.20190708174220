/*
 * Rustの高度な機能（関数、クロージャ）。
 * CreatedAt: 2019-07-08
 */
fn add_one(x: i32) -> i32 {
    x + 1
}
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}
fn main() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);
}
