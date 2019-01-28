
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("six is {:?}", six);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => {println!("failed"); None},
        Some(i) => {println!("{:?}", x);
                    Some(i + 1)},
    }
}
