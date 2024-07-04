<<<<<<< HEAD
// generics1.rs
//
// This shopping list program isn't compiling! Use your knowledge of generics to
// fix it.
//
// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let mut shopping_list: Vec<String> = Vec::new();
    shopping_list.push("milk".to_string());
=======
// `Vec<T>` is generic over the type `T`. In most cases, the compiler is able to
// infer `T`, for example after pushing a value with a concrete type to the vector.
// But in this exercise, the compiler needs some help through a type annotation.

fn main() {
    // TODO: Fix the compiler error by annotating the type of the vector
    // `Vec<T>`. Choose `T` as some integer type that can be created from
    // `u8` and `i8`.
    let mut numbers = Vec::new();

    // Don't change the lines below.
    let n1: u8 = 42;
    numbers.push(n1.into());
    let n2: i8 = -1;
    numbers.push(n2.into());

    println!("{numbers:?}");
>>>>>>> upstream/main
}
