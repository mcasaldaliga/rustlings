// strings3.rs
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    // first and last positions of non whitespace characters in input string
    let first:usize = 0;
    let last:usize = 0;
    // boolean var to know if we are past the white space beggining part of the string
    let middle = false;
    for (i, ch) in input.chars().enumerate() {
        if ch == ' ' {
            if ! middle {
                first += 1;
            } 
        } else {
            last = i;
        }
    }
    (&input[first..last]).to_string()
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    ???
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    ???
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
