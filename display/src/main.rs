use std::fmt;

// the way to declare a struct like this is similar to create a custom type in Golang
struct Structure(i32);
// example of create custom type in Rust::week date
#[allow(dead_code)]
struct WeekDay(i32);
// this also useful incase of type of a class need to assert

// implement Structure with fmt fn
impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.0)
    }
}

#[derive(Debug)]
struct Po {
    name:String,
    character:String,
}

impl fmt::Display for Po {
    fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"name: {}, character: {}",self.name,self.character)
    }
}

// field in a struct should be lower case + snake case
struct Info {
    name: String,
    age: u8,
}

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;

        write!(f,"[")?;
        for (count,v) in  vec.iter().enumerate() {
            if count != 0 {write!(f,", ")?;}
            write!(f,"{}: {}",count,v)?;
        }
        write!(f,"]")
    }
}

fn main() {
    let v = Structure(20);
    println!("{}",v);
    println!("{}",Structure(8));
    let info = Info{name:"khiem".to_string(),age:22};
    println!("{}\n{}",info.name,info.age);

    let po = Po{name:"po".to_string(),character:"animal".to_string()};
    println!("{}",po);

    let l = List(vec![1,2,3,4]);
    println!("{}",l);

    let mut inmmu = 6;
    inmmu += 1;
    println!("{}",inmmu);
    println!("{}",1000==1_000);

    println!("1 + 2 = {}",1u32+2);
    println!("1 - 2 = {}",1i32-2);
}
