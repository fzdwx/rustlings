// errors5.rs

// This program uses an altered version of the code from errors4.

// 这个练习使用了一些我们在课程后期才会用到的概念，
// 比如`Box`和`From`特性。现在详细了解它们并不重要，
// 但如果你愿意，可以提前阅读。
// 现在，把`Box<dyn ...>`类型看作是一个 "我想要做任何事情 "的类型，
// 考虑到Rust对运行时安全的通常标准，它应该让你觉得有点宽松！"。

// 简而言之，当你想拥有一个值，而你只关心它是一个实现了特定特质的类型时，盒子的这种特殊用例就会出现。
// 为了做到这一点，盒子被声明为Box<dyn Trait>类型，
// 其中Trait是编译器在该上下文中使用的任何值上寻找的特性。
// 在这个练习中，该上下文是可以在结果中返回的潜在错误。

// 我们可以用什么来描述这两种错误？换句话说，这两种错误是否都有一个特质？

// Execute `rustlings hint errors5` or use the `hint` watch subcommand for a hint.


use std::error;
use std::fmt;
use std::num::ParseIntError;

fn main() -> Result<(), Box<dyn error::Error>> {
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?;
    println!("output={:?}", PositiveNonzeroInteger::new(x)?);
    Ok(())
}

// Don't change anything below this line.

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

// This is required so that `CreationError` can implement `error::Error`.
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError::Negative => "number is negative",
            CreationError::Zero => "number is zero",
        };
        f.write_str(description)
    }
}

impl error::Error for CreationError {}
