#[derive(Debug)]
enum UsState {
    Ohio,
    Newyork,
}

#[derive(Debug)]
enum Coin {
    Beeny,
    Jeene(UsState),
}

fn get_value_coin(coin: Coin) -> u8 {
    match coin {
        Coin::Beeny => 5,
        Coin::Jeene(state) => {
            println!("jennce state: {:?}", state);
            10
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}

fn main() {
    let state = UsState::Newyork;
    let coin = Coin::Jeene(state);
    let coin_value = get_value_coin(coin);
    println!("value of coin: {}", coin_value);
    let five = Some(5);
    let six = plus_one(five);
    println!("value of six: {:?}", six);
}
