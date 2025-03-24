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
}
