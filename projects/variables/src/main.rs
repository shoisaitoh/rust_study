fn main() {
    //let mut x = 5;
    //println!("The value of x is {}", x);
    //x = 6;
    //println!("The value of x is {}", x);

    let x = 5;

    let x = x + 1;

    {
        let x = x + 2;
        println!("The value of x in the inner scope is {}", x);
    }

    println!("The value of x is {}", x);


    // ok
    //let spaces = "    ";
    //let spaces = spaces.len();
    //println!("The value of spaces is {}", spaces);

    // ng, mutableだから
    //let mut spaces = "    ";
    //let spaces = spaces.len();
    //println!("The value of spaces is {}", spaces);


    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is {}", guess);

    // 浮動小数点数はf64がデフォルトで精度が高い
    // let x = 2.0; // f64
    // let y: f32 = 3.0; // f32

    // 除算に注意
    let floored = 2 / 3; // 結果は0になる
    println!("The value of floored is {}", floored);

    // 余り出す
    let reminder = 43 % 5; // 3
    println!("The value of reminder is {}", reminder);

    // boolean
    // let t = true;
    // let f: bool = false; // 明示的型注釈付き

    // 文字型≠文字列
    // シングルクォートとダブルクォートは区別される
    // シングル→文字型(char)
    // ダブル→文字列(string)

    // let c = 'z'

    // タプル
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_xx, yy, _zz) = tup;
    // 未使用の変数は、cargo run時にwarningが出る
    // これを回避するには、変数の頭にアンダースコアをつける
    println!("The value of y is {}", yy);


}
