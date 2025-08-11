use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    let rng = rand::rng().random_range(0..=100);
    println!("当前随机数：{}", rng);
    loop {
        println!("请输入一个随机数");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取行");

        println!("你猜测的数是：{}", guess);

        // let guess: u32 = guess.trim().parse().expect("转换异常，请输入一个整数");
        let guess: u32 = match guess.trim().parse() {
            Ok(num  )=>num,
            Err(_) => {
                println!("输入无效，请输入一个整数");
                continue;
            }
        };

        match guess.cmp(&rng) {
            Ordering::Less => println!("太小"),
            Ordering::Greater => println!("太大"),
            Ordering::Equal => {
                println!("猜对了");
                break;
            },
        }
    }
}
