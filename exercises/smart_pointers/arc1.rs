// arc1.rs
// 在这个练习中，我们得到了一个名为 "数字 "的u32的Vec，其数值范围是0到99 -- [ 0, 1, 2, ..., 98, 99 ] 。
// 我们想在8个不同的线程中同时使用这组数字。
// 每个线程都将获得每八位数值的总和，并有一个偏移量。
// 第一个线程（偏移量0），将对0、8、16、...进行求和。
// 第二个线程（偏移量1），将得到1, 9, 17, ...
// 第三个线程（偏移量2），将求出2，10，18，...
// ...
// 第八个线程（偏移量7），将对7、15、23、...

// Because we are using threads, our values need to be thread-safe.  Therefore,
// we are using Arc.  We need to make a change in each of the two TODOs.



// Execute `rustlings hint arc1` or use the `hint` watch subcommand for a hint.


#![forbid(unused_imports)] // Do not change this, (or the next) line.
use std::sync::Arc;
use std::thread;

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();
    let shared_numbers = Arc::new(numbers);
    let mut joinhandles = Vec::new();

    for offset in 0..8 {
        let child_numbers = shared_numbers.clone();
        joinhandles.push(thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|n| *n % 8 == offset).sum();
            println!("Sum of offset {} is {}", offset, sum);
        }));
    }
    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }
}
