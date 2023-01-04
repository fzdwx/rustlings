// traits3.rs
//
// 你的任务是为这两个结构实现Licensed特性，并让它们返回相同的信息，而不写两次相同的函数。

// Consider what you can add to the Licensed trait.
// Execute `rustlings hint traits3` or use the `hint` watch subcommand for a hint.


pub trait Licensed {
    fn licensing_info(&self) -> String{
        String::from("Some information")
    }
}

struct SomeSoftware {
    version_number: i32,
}

struct OtherSoftware {
    version_number: String,
}

impl Licensed for SomeSoftware {} // Don't edit this line
impl Licensed for OtherSoftware {} // Don't edit this line

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_licensing_info_the_same() {
        let licensing_info = String::from("Some information");
        let some_software = SomeSoftware { version_number: 1 };
        let other_software = OtherSoftware {
            version_number: "v2.0.0".to_string(),
        };
        assert_eq!(some_software.licensing_info(), licensing_info);
        assert_eq!(other_software.licensing_info(), licensing_info);
    }
}
