use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    println!("猜数！");

    // 生成随机数
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        // 输入提示
        println!("猜一个数：");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("msg");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // 输入出用户的输入内容
        println!("你猜的数是：{}", guess);

        // 判断是否正确
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
        }
    }
}
