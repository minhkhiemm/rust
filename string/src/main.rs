fn main() {
    let s1 = String::from("hello");
    let s2 = String::from("world");
    s1.push(s2);
    println!("s2:{}", s2);
    println!("s1: {}", s1)
}
