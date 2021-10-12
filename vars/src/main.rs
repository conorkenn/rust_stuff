fn main() {
    let x = fib(9);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

//celsius to fahrenheit
fn temp(x: f64) -> f64 {
     (x * (9.0/5.0)) + 32.0
}

fn fib(x: i64) -> i64{
    if x <= 1{
        return x;
    }
    fib(x -1) + fib(x -2)
}