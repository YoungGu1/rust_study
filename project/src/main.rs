fn main() {
    println!("Hello, world!");
    test();
    test_square();
}

fn test() {
    let nice_count: i32 = 100;
    let nice_number: i64 = 54;

    println!("nice_count :{}", nice_count);
    println!("nice_number :{}", nice_number);

    // nice_count = 20;   不能修改

    let mut count = 3;
    count = count + 1;
    println!("count :{}", count);

    let mut x = "hello";
    println!("x : {}", x);

    x = "world";
    println!("x : {}", x);

}

// 正方形结构体
#[derive(Debug, Clone, Copy)]
struct Square {
    side_length: f64,
}

impl Square {
    // 构造函数
    fn new(side_length: f64) -> Square {
        Square { side_length }
    }
    
    // 计算面积
    fn area(&self) -> f64 {
        self.side_length * self.side_length
    }
    
    // 计算周长
    fn perimeter(&self) -> f64 {
        4.0 * self.side_length
    }
    
    // 获取边长
    fn get_side_length(&self) -> f64 {
        self.side_length
    }
    
    // 设置边长
    fn set_side_length(&mut self, new_side_length: f64) {
        self.side_length = new_side_length;
    }
    
    // 缩放正方形
    fn scale(&mut self, factor: f64) {
        self.side_length *= factor;
    }
}

// 测试函数
fn test_square() {
    println!("\n=== 正方形测试 ===");
    
    // 创建一个边长为5的正方形
    let mut square = Square::new(5.0);
    println!("创建正方形: {:?}", square);
    println!("边长: {}", square.get_side_length());
    println!("面积: {}", square.area());
    println!("周长: {}", square.perimeter());
    
    // 修改边长
    square.set_side_length(8.0);
    println!("\n修改边长为8.0后:");
    println!("边长: {}", square.get_side_length());
    println!("面积: {}", square.area());
    println!("周长: {}", square.perimeter());
    
    // 缩放正方形
    square.scale(1.5);
    println!("\n缩放1.5倍后:");
    println!("边长: {}", square.get_side_length());
    println!("面积: {}", square.area());
    println!("周长: {}", square.perimeter());
    
    // 创建多个正方形进行比较
    let square1 = Square::new(3.0);
    let square2 = Square::new(4.0);
    
    println!("\n比较两个正方形:");
    println!("正方形1 - 边长: {}, 面积: {}", square1.get_side_length(), square1.area());
    println!("正方形2 - 边长: {}, 面积: {}", square2.get_side_length(), square2.area());
}
