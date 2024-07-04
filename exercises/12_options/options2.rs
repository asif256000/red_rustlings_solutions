<<<<<<< HEAD
// options2.rs
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a hint.
=======
fn main() {
    // You can optionally experiment here.
}
>>>>>>> upstream/main

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

<<<<<<< HEAD
        // TODO: Make this an if let statement whose value is "Some" type
        if let word = optional_target {
            assert_eq!(word, Some(target));
=======
        // TODO: Make this an if-let statement whose value is `Some`.
        word = optional_target {
            assert_eq!(word, target);
>>>>>>> upstream/main
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = Vec::new();

        for i in 1..=range {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

<<<<<<< HEAD
        // TODO: make this a while let statement - remember that vector.pop also adds another layer of Option<T>
        // You can stack `Option<T>`'s into while let and if let
        while let Some(integer) = optional_integers.pop() {
            assert_eq!(integer, Some(cursor));
=======
        // TODO: Make this a while-let statement. Remember that `Vec::pop()`
        // adds another layer of `Option`. You can do nested pattern matching
        // in if-let and while-let statements.
        integer = optional_integers.pop() {
            assert_eq!(integer, cursor);
>>>>>>> upstream/main
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}
