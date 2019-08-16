fn main() {
    // variables in Rust mutate by default
    // let x = 5;
    // println!("{}", x);
    // let addx = &x;
    // println!("{:p}", addx);
    // // that mean it can not be change
    // // the let x below create new memory address for x
    // let x = x + 1;
    // println!("{}", x);
    // let addx = &x;
    // println!("{:p}", addx);
    // variables can be free from mutation by:
    let mut x = 5;
    println!("{}", x);
    x = 6;
    println!("{}", x);
    let addx = &mut x;
    println!("{:?}", addx);
}
