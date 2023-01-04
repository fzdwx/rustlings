// options1.rs
// Execute `rustlings hint options1` or use the `hint` watch subcommand for a hint.


// 这个函数返回冰箱里还剩下多少冰激凌。如果是在10点之前，就剩下5块。到了10点，有人把它们吃光了，所以就没有了:(
fn maybe_icecream(time_of_day: u16) -> Option<u16> {
    // 我们在这里使用的是 24 小时制，所以 10 点的值是 22，12 点的值是 0。
    // Option的输出应该优雅地处理time_of_day>23的情况。
    // TODO: Complete the function body - remember to return an Option!
    if time_of_day < 22 {
        Some(5)
    } else if time_of_day > 24 {
        None
    } else {
        Some(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(10), Some(5));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(25), None);
    }

    #[test]
    fn raw_value() {
        // TODO: Fix this test. How do you get at the value contained in the Option?
        let icecreams = maybe_icecream(12);
        assert_eq!(icecreams, Some(5));
    }
}
