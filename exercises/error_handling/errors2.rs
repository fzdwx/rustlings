// errors2.rs
// 假设我们正在编写一个游戏，你可以用代币购买物品。
// 所有物品都需要5个代币，每当你购买物品时，都会有1个代币的处理费。
// 游戏的玩家将输入他们想要购买的物品数量，`total_cost`函数将计算出代币的总数量。
// 由于玩家输入的是数量，我们得到的是一个字符串--他们可能输入了任何东西，而不仅仅是数字!

// 现在，这个函数根本没有处理错误的情况（也没有正确处理成功的情况）。
// 我们要做的是：如果我们在一个不是数字的字符串上调用`parse`函数，
// 该函数将返回一个`ParseIntError`，在这种情况下，
// 我们要立即从我们的函数中返回这个错误，而不是试图进行乘加。

// 至少有两种方法可以实现这一点，它们都是正确的--但其中一种方法要短得多。

// Execute `rustlings hint errors2` or use the `hint` watch subcommand for a hint.


use std::num::ParseIntError;

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().to_string(),
            "invalid digit found in string"
        );
    }
}
