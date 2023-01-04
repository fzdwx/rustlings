// box1.rs
//
// 在编译时，Rust需要知道一个类型占用了多少空间。
// 这对于递归类型来说是有问题的，因为一个值本身可以有另一个相同类型的值作为它的一部分。
// 为了解决这个问题，我们可以使用`Box`--一个用于在堆上存储数据的智能指针，
// 它也允许我们包裹一个递归类型。
//
// 我们在这个练习中要实现的递归类型是 "cons list"--一种在函数式编程语言中经常出现的数据结构。
// cons list中的每一项都包含两个元素：
// 当前项的值和下一项的值。
// 最后一项是一个叫做 "Nil "的值。
//
// Step 1: 在枚举定义中使用 `Box` 使代码编译
// Step 2: 通过替换 `todo!()` 创建空的和非空的 cons list
//
// Note: the tests should not be changed
//
// Execute `rustlings hint box1` or use the `hint` watch subcommand for a hint.

#[derive(PartialEq, Debug)]
pub enum List {
    Cons(Box<(i32, List)>),
    Nil,
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list()
    );
}

pub fn create_empty_list() -> List {
    List::Nil
}

pub fn create_non_empty_list() -> List {
    List::Cons(Box::new((0, List::Nil)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(List::Nil, create_empty_list())
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list())
    }
}
