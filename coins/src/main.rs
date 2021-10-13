#[derive(Debug)] 


enum State{
    Alaska,
    Alabama,

}

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(State),
}

fn main() {
    value_in_cents(Coin::Quarter(State::Alaska));
    let x = value_in_cents(Coin::Nickel);
    println!("{}",x);
}

fn value_in_cents(coin: Coin)-> u8{
    match coin{
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        },
    }
}