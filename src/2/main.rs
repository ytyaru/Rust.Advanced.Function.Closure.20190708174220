/*
 * Rustの高度な機能（関数、クロージャ）。
 * CreatedAt: 2019-07-08
 */
fn main() {
}
/*
// error[E0277]: the size for values of type `(dyn std::ops::Fn(i32) -> i32 + 'static)` cannot be known at compilation time
fn returns_closure() -> Fn(i32) -> i32 {
    |x| x + 1
}
*/
fn returns_closure() -> Box<Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
