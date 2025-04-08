use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("guess the number!");

    //生成一个随机数
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("please input your guess.");

        //声明一个可变的变量来存储用户输入的值
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        //将用户输入的值转换为数字
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guessed: {}", guess);

        //比较用户输入的值和随机数的大小
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too big!"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
        }
    }
}
