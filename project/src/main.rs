fn main() {
    println!("Hello, world!");
    test();
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
