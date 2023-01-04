// modules3.rs
// 你可以使用'use'关键字将任何地方的模块，特别是Rust标准库中的模块路径引入你的范围。
// 从std::time模块中引入SystemTime和UNIX_EPOCH。
// 如果你能用一行就完成的话，就会有额外的风格加分。
// Execute `rustlings hint modules3` or use the `hint` watch subcommand for a hint.


use std::time::{SystemTime,UNIX_EPOCH};


fn main() {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
