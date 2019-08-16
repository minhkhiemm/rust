#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }

    fn get_y(&self) -> &T {
        &self.y
    }
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let nums = vec![1, 2, 3, 4, 5];
    let result = largest(&nums);
    println!("{}", result);
    let chares = vec!['a', 'd', 'm', 'z'];
    let car = largest(&chares);
    println!("{}", car);
    let point = Point {x:1,y:2};
    let x = point.get_x();
    println!("value of x: {}",x);
    let y = point.get_y();
    println!("value of y: {}",y)
}
