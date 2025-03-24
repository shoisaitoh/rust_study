// cf. https://doc.rust-jp.rs/book-ja/ch02-00-guessing-game-tutorial.html
// 標準ライブラリの中でも、だいたいのプログラムで使われるものは、プレリュードとして
// 特に宣言しなくても使える
// そうでないものは、use文で明示する
use rand::Rng; // randクレートはライブラリクレート、単独で実行できない
use std::cmp::Ordering;
// Orderingもenumのひとつで、列挙子としてLess,Greater,Equalを持つ
use std::io; // std::ioをuseする、標準ライブラリの入出力機能

fn main() {
    println!("Guess the number!"); // 数当て

    let secret_number = rand::thread_rng().gen_range(1..101);
    // rand::thread_rng()関数→このスレッドに固有の、これから使う特定の乱数生成器
    // gen_range()関数→乱数を指定された範囲内で生成する関数
    // gen_range(1..101)とgen_range(1..=100)は等価

    //println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        // Rustにおいて、変数はデフォルトで不変（イミュータブル）
        // ただし、let mutとすると、変数を可変（ミュータブル）にできる
        //let apples = 5; immutable, 不変
        let mut guess = String::new(); // mutable, 可変
        // String::new()は、String型の空のインスタンス

        io::stdin() // 標準入力をする
            .read_line(&mut guess) // 1行読み込んで、ユーザからの入力をguess変数に入れる
            // &はこの引数が参照であることを示し、
            // コードで何度も参照しても同じメモリから？使う
            // 参照を安全かつ簡単に使用できるのがRustのメリットの一つ
            .expect("Failed to read line"); // 行の読み込みに失敗しました
            // read_line関数は、Result型という列挙型（enum）も返す
            // Resultの列挙子はOkとErr
            // expect関数は、Errが返ってきたときにプログラムをクラッシュさせる

        // u32→符号なしの32bit数
            let guess: u32 = match guess.trim().parse() {
            // guessの型をString型からu32型に変更できる
            // trim()→文字列の前後の空白を削除
            // parse()→文字列をパース（解析）してなんらかの数値に
            // その際、u32などと型指定が必要

            // エラー処理
            // parse式が文字列からu32の数値型に変換成功した場合、parse式が
            // 返したnumをそのままguess変数に入れる
            Ok(num) => num, // 数値を入力してもらったら、進む
            // アンダースコアはすべての値を受けつける
            Err(_) => continue, // 数値以外だったら、continue文で再入力
        };

        println!("You guessed: {}", guess);
        // let x = 5
        // let y = 10
        // println!("x = {}, y = {}", x, y)

        match guess.cmp(&secret_number) {
            // cmp関数→2つの値を比較する→ここでは変数guessとsecret_numberを比較
            // 複数の=>はmatch式、順に照合していく、（条件）=>（一致した時の処理）
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You Win!");
                break; // break文でloopを抜ける
            }
        }
    }
}
