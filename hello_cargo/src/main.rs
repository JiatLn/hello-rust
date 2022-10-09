use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let num = rand::thread_rng().gen_range(1..=10);

    for i in (1..4).rev() {
        println!("{}", i);
    }

    println!("请输入一个数字");

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("读取行失败");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&num) {
            Ordering::Less => println!("太小了"),
            Ordering::Greater => println!("太大了"),
            Ordering::Equal => {
                println!("猜对了");
                println!("随机生成的数字是: {}", num);
                break;
            }
        }
        println!("你输入的数字是: {}", guess);
    }
}
