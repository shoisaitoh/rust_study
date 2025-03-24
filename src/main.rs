// https://www.rust-lang.org/ja/learn/get-started
use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();
    
    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();

    // loopでアンダーバーをつけろって"cargo check"で言われたんだけど
    // これはいったい何？
    for _n in 0.. 10 {
        println!("Hello, World!");
    }
}
