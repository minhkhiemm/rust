#[derive(Debug)]
enum TShirt {
    Size(i32),
    Name(String),
}

fn main() {
    let vet_tshirt = vec![TShirt::Size(32), TShirt::Name(String::from("box tee"))];
    println!(
        "the tshirt name: {:?}, the tshirt type: {:?}",
        vet_tshirt[0], vet_tshirt[1]
    )
}
