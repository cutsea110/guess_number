extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    //println!("secret number is: {}", secret_number);

    let mut count = 1;

    loop {
        println!("かずをあててみて");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("すうじをいれてよ");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("1から100までのかずしかいれないで (>_<)");
                continue;
            }
        };

        println!("{} かいめ: あなたのよそう: {}", count, guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("ちいさすぎー! q(6_6)");
                count += 1;
            }
            Ordering::Greater => {
                println!("おおきすぎー! (6_6)b");
                count += 1;
            }
            Ordering::Equal => {
                println!(
                    "あったりー!: {} かいめであてたよ (^-^)v",
                    count
                );
                if count == 7 {
                    println!("よくがんばったね!");
                } else if count > 7 {
                    println!("ざんねん! つぎはもっとがんばろうね.");
                } else if count < 7 {
                    println!("すっごーい! やったねー! \\(^o^)/");
                }
                break;
            }
        }
    }
}
