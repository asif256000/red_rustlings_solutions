<<<<<<< HEAD
// options3.rs
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a hint.

=======
#[derive(Debug)]
>>>>>>> upstream/main
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let optional_point = Some(Point { x: 100, y: 200 });

<<<<<<< HEAD
    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => println!("no match"),
=======
    // TODO: Fix the compiler error by adding something to this match statement.
    match optional_point {
        Some(p) => println!("Co-ordinates are {},{}", p.x, p.y),
        _ => panic!("No match!"),
>>>>>>> upstream/main
    }

    println!("{optional_point:?}"); // Don't change this line.
}
