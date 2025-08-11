pub struct People {
    pub name: String,
    pub age: u32,
}

impl People {
    pub fn new(name: String, age: u32) -> Self {
        People { name, age }
    }
    
    pub fn introduce(&self) {
        println!("Hello, I'm {} and I'm {} years old.", self.name, self.age);
    }
    
}