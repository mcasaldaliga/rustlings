// quiz2.rs
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums

// Let's build a little machine in form of a function.
// As input, we're going to give a list of strings and commands. These commands
// determine what action is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
// No hints this time!


pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: Complete the function signature!
    pub fn transformer(input: Vec::<(String, Command)>) -> Vec::<String> {
        // TODO: Complete the output declaration!
        let mut output: Vec::<String> = vec![];
        for (string, command) in input.iter() {
            let mut out = string.clone();
            match command {
                Command::Uppercase => {
                    out = out.to_uppercase();
                },
                Command::Trim => {
                    out = trim_me(&out);
                },
                Command::Append(times) => {
                    for i in 0..*times {
                        out.push_str("bar");
                    }
                },
            }
            output.push(out);
        }
        output
    }

    fn trim_me(input: &str) -> String {
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
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
