// move_semantics4.rs
// 重构这段代码，使其不再将`vec0`传入`fill_vec`函数，而是在该函数中创建向量，并传回给主函数。
// Execute `rustlings hint move_semantics4` or use the `hint` watch subcommand for a hint.


fn main() {
    let mut vec1 = fill_vec();

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// `fill_vec()` no longer takes `vec: Vec<i32>` as argument
fn fill_vec() -> Vec<i32> {
    let mut vec = vec![];

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
