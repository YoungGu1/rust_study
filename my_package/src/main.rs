
//声明一个模块people  会去找people.rs文件
//如果没有people.rs文件会去找people/mod.rs文件
//如果没有people/mod.rs文件会去找people目录下的lib.rs文件
mod people;

// 引入people模块中的People结构体
use people::People;

fn main() {
    let person = People::new("张三".to_string(), 25);
    person.introduce();
    
    let another_person = People::new("李四".to_string(), 30);
    another_person.introduce();

}
