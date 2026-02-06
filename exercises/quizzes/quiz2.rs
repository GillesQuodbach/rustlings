// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
//
// The exact form of this will be:
// - The input is going to be a Vector of 2-length tuples,
//   the first element is the string, the second one is the command.
// - The output element is going to be a vector of strings.

#[derive(Debug)]
pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: Complete the function as described above.
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {

        input
            .into_iter()
            .map(|(mut s, cmd)| match cmd {
                Command::Uppercase => s.to_uppercase(),
                Command::Trim => s.trim().to_owned(),
                Command::Append(n) => {
                    // Append "bar" n fois à la fin
                    // Pré-allocation: on sait qu'on ajoute 3*n caractères
                    s.reserve(3 * n);
                    for _ in 0..n {
                        s.push_str("bar");
                    }
                    s
                }
            })
            .collect()
    }
    //     let mut output_array = Vec::new();
    //     for (s, cmd) in &input {
    //         println!("reçu: {:?} + {:?}", s, cmd);
    //         match cmd {
    //             Command::Uppercase => {
    //                 let new_string = s.to_uppercase();
    //                 output_array.push(new_string);
    //             }
    //             Command::Trim => {
    //                 let new_string = s.trim().to_string();
    //                 output_array.push(new_string);
    //             }
    //             Command::Append(n) => {
    //                 let mut count = 0;
    //                 let mut output_string = String::new();
    //                 let mut base_string = String::new();
    //                 base_string.push_str(&s.to_string());
    //                 println!("{}", base_string);
    //                 if base_string == "foo" {
    //
    //                     let bar_string = String::from("bar");
    //                     let mut foobar_string = String::from(&base_string);
    //                     foobar_string.push_str(&bar_string);
    //                     output_array.push(foobar_string);
    //
    //                 } else {
    //                     while count <= *n {
    //                         output_string.push_str(&base_string);
    //                         count += 1;
    //                     }
    //                     output_array.push(output_string);
    //                 }
    //
    //             }
    //         }
    //     }
    //     output_array
    // }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    use crate::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(input);

        assert_eq!(
            output,
            [
                "HELLO",
                "all roads lead to rome!",
                "foobar",
                "barbarbarbarbarbar",
            ]
        );
    }
}
