// enum IpAddr {
//     V4(String),
//     V6(String),
// }

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    // let home = IpAddr::V4(String::from("127.0.0.1"));
    // let loopback = IpAddr::V6(String::from("::1"));
    let my_coin = Coin::Penny;
    println!("My coin kind is {:?}", my_coin);
    println!("My coin value is {}", value_in_cents(&my_coin));
    println!("My coin kind is {:?}", my_coin);

    let state_coin = Coin::Quarter(UsState::Alabama);
    println!("My coin value is {}", value_in_cents(&state_coin));

    println!("==============");
    let five = Some(5);
    let six = plus_one(five);
    println!("{:?}", six);
    let none = plus_one(None);
    println!("{:?}", none);
}

fn value_in_cents(coin: &Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
