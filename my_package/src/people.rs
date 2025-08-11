//结构体 有两个公开的字段 姓名和年龄
pub struct People {
    pub name: String,
    pub age: u32,
}

//
impl People {
    // 创建一个新的People实例 构造方法
    pub fn new(name: String, age: u32) -> Self {
        People { name, age }
    }
    
    pub fn introduce(&self) {
        println!("Hello, I'm {} and I'm {} years old.", self.name, self.age);
    }
    
}