// hashmaps1.rs
// 需要定义一个哈希图形式的水果篮。键代表水果的名称，值代表篮子里有多少个这种特定的水果。你必须把
// 篮子里至少有三种不同类型的水果（如apple, banana, mango），所有水果的总数量至少为5。
//
// Make me compile and pass the tests!
//
// Execute `rustlings hint hashmaps1` or use the `hint` watch subcommand for a hint.


use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    let mut basket = HashMap::new();

    // Two bananas are already given for you :)
    basket.insert(String::from("banana"), 2);

    basket.insert(String::from("apple"), 10);
    basket.insert(String::from("mango"), 10);
    basket
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}
