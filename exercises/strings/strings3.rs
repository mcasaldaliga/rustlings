// strings3.rs
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

pub fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    // first and last positions of non whitespace characters in input string
    let mut first:usize = 0;
    let mut last:usize = 0;
    // boolean var to know if we are past the white space beggining part of the string
    let mut middle = false;
    for (i, ch) in input.chars().enumerate() {
        if ch == ' ' {
            if ! middle {
                first += 1;
            } 
        } else {
            middle = true;
            last = i;
        }
    }
    (&input[first..(last + 1)]).to_string()
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    format!("{}{}",input.to_string(), " world!".to_string())
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    if let Some(idx) = input.find("cars") {
        format!("{}balloons{}", &input[..idx].to_string(),
                &input[(idx + 4)..].to_string())
    } else {
        input.to_string()
    }
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
