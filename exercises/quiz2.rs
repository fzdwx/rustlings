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

// 让我们以一个函数的形式建立一个小机器。 作为输入，我们将给出一个字符串和命令的列表。
// 这些命令决定了要对字符串采取什么行动。它可以是:
// - 对字符串进行大写字母处理
// - 修剪字符串
// - 将 "bar "添加到字符串中指定的次数。
// 具体的形式是。
// - 输入将是一个2长元组的向量。
// 第一个元素是字符串，第二个元素是命令。
// - 输出元素将是一个字符串的向量。
// 这次没有提示!


pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        let mut output: Vec<String> = vec![];
        for (string, command) in input.iter() {
            let string: String = match command {
                Command::Uppercase => {
                    (&string).to_uppercase()
                }
                Command::Trim => {
                    (*string).trim().to_string()
                }
                Command::Append(times) => {
                    let mut string = string.clone();
                    string.push_str(&"bar".repeat(*times));
                    string
                }
            };
            output.push(string);
        }
        output
    }
}

#[cfg(test)]
mod tests {
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
