// if1.rs
// Execute `rustlings hint if1` or use the `hint` watch subcommand for a hint.


pub fn bigger(a: i32, b: i32) -> i32 {
    // 完成这个函数，返回更大的数字!
    // 切勿使用:
    // - 另一个函数调用
    // - 附加变量
    if a > b {
        a
    } else {
        b
    }
}

// Don't mind this for now :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }
}
