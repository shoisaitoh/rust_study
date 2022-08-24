use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    // gen_range(1..101)とgen_range(1..=100)は等価

    //println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        //let apples = 5; immutable, 不変
        let mut guess = String::new(); // mutable, 可変

        io::stdin()
            .read_line(&mut guess) // &はこの引数が参照であることを示し、コードで何度も参照しても同じメモリから？使う
            // 参照を安全かつ簡単に使用できるのがRustのメリットの一つ
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        // let x = 5
        // let y = 10
        // println!("x = {}, y = {}", x, y)

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
