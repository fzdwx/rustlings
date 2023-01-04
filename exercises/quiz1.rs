// quiz1.rs
// This is a quiz for the following sections:
// - Variables
// - Functions
// - If

// 玛丽正在买苹果。一个苹果的价格计算如下:
// - 一个苹果的价格是2锈币.
// - 如果玛丽买了40个以上的苹果，每个苹果只需花费1个锈币!
// 编写一个函数，在给定购买数量的情况下计算苹果订单的价格。
// 这次没有提示！


// Put your function here!
fn calculate_price_of_apples(num: i32) -> i32 {
    let mut price = 2;
    if num > 40 {
        price = 1;
    }
    price * num
}

// Don't modify this function!
#[test]
fn verify_test() {
    let price1 = calculate_price_of_apples(35);
    let price2 = calculate_price_of_apples(40);
    let price3 = calculate_price_of_apples(41);
    let price4 = calculate_price_of_apples(65);

    assert_eq!(70, price1);
    assert_eq!(80, price2);
    assert_eq!(41, price3);
    assert_eq!(65, price4);
}
