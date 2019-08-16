pub trait Summary {
    fn summary(&self) -> String;
}

#[derive(Debug)]
struct Tweet {
    author: String,
    name: String,
    content: String,
}

impl Summary for Tweet {
    fn summary(&self) -> String {
        format!("tweet name: {} of author: {}, have content: {}",self.name,self.author,self.content)
    }
}

fn main() {
    let t = Tweet{author:String::from("khiem"),name:String::from("rustdocs"),content:String::from("awsome rust documentations")};
    println!("{}",t.summary())
}
