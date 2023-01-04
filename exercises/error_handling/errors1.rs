// errors1.rs
// 如果您将空字符串传递给该函数，该函数将拒绝生成要打印在名称标签上的文本。如果它能解释问题所在，
// 而不是有时只返回“无”，那就更好了。
// 值得庆幸的是，Rust 有一个类似于 `Option` 的结构，可以用来表达错误条件。让我们使用它！
// Execute `rustlings hint errors1` or use the `hint` watch subcommand for a hint.

pub fn generate_nametag_text(name: String) -> Result<String,String> {
    if name.is_empty() {
        // Empty names aren't allowed.
        Err("`name` was empty; it must be nonempty.".into())
    } else {
        Ok(format!("Hi! My name is {}", name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".into()),
            Ok("Hi! My name is Beyoncé".into())
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text("".into()),
            // Don't change this line
            Err("`name` was empty; it must be nonempty.".into())
        );
    }
}
