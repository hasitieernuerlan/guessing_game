use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("猜猜数字!");
    let secret_number = rand::thread_rng().gen_range(1,10);

    loop{
        println!("请输入1-10之间的数字，胜利的条件是你猜对保密数字");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
    .expect("错误无法读取！");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    // .expect("请输入一个数字，位于1到10之间！");
    
    println!("你输入的数字是:{}",guess);
    
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("你的数字太小了！再来！"),
        Ordering::Greater => println!("你的数字太大了！再来！"),
        // Ordering::Equal => println!("你猜对了！你赢了！"),
        Ordering::Equal => {
            println!("你猜对了！你赢了！");
            break;
        },

    }
    
    // println!("保密数字是:{}",secret_number); 
    }
    
   
}