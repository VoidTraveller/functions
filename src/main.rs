fn main() {
    println!("Hello, world!");

    let x = 5 + 6;
    println!("{x}");

    another_function(5);
}

fn another_function(x: i32) {
    println!("Value of x is: {x}");
}