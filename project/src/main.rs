mod people;

use project::Square;
use people::People;

fn main() {
    println!("Hello, world!");
    test();
    test_lib_square();
    test_people();
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

// 使用lib中的Square结构体进行测试
fn test_lib_square() {
    println!("\n=== 调用lib中的正方形测试 ===");
    
    // 创建正方形，使用Result处理
    match Square::new(5.0) {
        Ok(mut square) => {
            println!("创建正方形: {:?}", square);
            println!("边长: {}", square.get_side_length());
            println!("面积: {}", square.area());
            println!("周长: {}", square.perimeter());
            
            // 修改边长
            if let Ok(()) = square.set_side_length(8.0) {
                println!("\n修改边长为8.0后:");
                println!("边长: {}", square.get_side_length());
                println!("面积: {}", square.area());
                println!("周长: {}", square.perimeter());
            }
            
            // 缩放正方形
            if let Ok(()) = square.scale(1.5) {
                println!("\n缩放1.5倍后:");
                println!("边长: {}", square.get_side_length());
                println!("面积: {}", square.area());
                println!("周长: {}", square.perimeter());
            }
            
            // 测试单位正方形
            println!("是否为单位正方形: {}", square.is_unit_square());
        }
        Err(e) => println!("创建正方形失败: {}", e),
    }
    
    // 创建多个正方形进行比较
    if let (Ok(square1), Ok(square2)) = (Square::new(3.0), Square::new(4.0)) {
        println!("\n比较两个正方形:");
        println!("正方形1 - 边长: {}, 面积: {}", square1.get_side_length(), square1.area());
        println!("正方形2 - 边长: {}, 面积: {}", square2.get_side_length(), square2.area());
        println!("两个正方形相等吗: {}", square1 == square2);
    }
    
    // 测试错误情况
    println!("\n测试错误处理:");
    match Square::new(-5.0) {
        Ok(_) => println!("不应该成功创建负边长的正方形"),
        Err(e) => println!("预期的错误: {}", e),
    }
    
    // 测试单位正方形
    if let Ok(unit_square) = Square::new(1.0) {
        println!("\n单位正方形测试:");
        println!("是否为单位正方形: {}", unit_square.is_unit_square());
        println!("单位正方形面积: {}", unit_square.area());
        println!("单位正方形周长: {}", unit_square.perimeter());
    }
}

// 测试独立的People结构体
fn test_people() {
    println!("\n=== 独立People模块测试 ===");
    
    // 创建第一个人物实例（包含职业）
    match People::new(
        "张小明".to_string(),
        25,
        "zhangxiaoming@qq.com".to_string(),
        "北京".to_string(),
        "软件工程师".to_string()
    ) {
        Ok(mut person) => {
            println!("✅ 成功创建人物:");
            println!("{}", person.get_full_info());
            
            // 调用自我介绍方法
            println!("\n📢 自我介绍:");
            println!("{}", person.introduce());
            
            // 庆祝生日
            println!("\n🎂 生日庆祝:");
            let birthday_msg = person.celebrate_birthday();
            println!("{}", birthday_msg);
            
            // 修改信息
            println!("\n📝 信息更新:");
            if let Ok(()) = person.set_city("上海".to_string()) {
                println!("  ✅ 城市更新成功: {}", person.get_city());
            }
            
            person.set_occupation("高级软件工程师".to_string());
            println!("  ✅ 职业更新成功: {}", person.get_occupation());
            
            if let Ok(()) = person.set_email("xiaoming.zhang@company.com".to_string()) {
                println!("  ✅ 邮箱更新成功: {}", person.get_email());
            }
        }
        Err(e) => println!("❌ 创建人物失败: {}", e),
    }
    
    // 创建多个人物进行交互测试
    println!("\n\n=== 人物交互测试 ===");
    let people_data = vec![
        ("小红", 8, "xiaohong@example.com", "北京", "小学生"),
        ("小李", 16, "xiaoli@example.com", "北京", "高中生"),
        ("王先生", 35, "wangxiansheng@example.com", "上海", "医生"),
        ("李奶奶", 72, "linainai@example.com", "成都", "退休教师"),
    ];
    
    let mut people_list = Vec::new();
    
    for (name, age, email, city, occupation) in people_data {
        if let Ok(person) = People::new(
            name.to_string(),
            age,
            email.to_string(),
            city.to_string(),
            occupation.to_string(),
        ) {
            println!("\n👤 {}:", person.get_name());
            println!("   年龄: {}岁 ({})", person.get_age(), person.get_age_group());
            println!("   职业: {}", person.get_occupation());
            println!("   城市: {}", person.get_city());
            println!("   退休倒计时: {}年", person.years_to_retirement());
            
            people_list.push(person);
        }
    }
    
    // 人物之间的交互
    if people_list.len() >= 2 {
        println!("\n\n=== 人物互动 ===");
        
        // 同城检测和问候
        for i in 0..people_list.len() {
            for j in (i+1)..people_list.len() {
                let person1 = &people_list[i];
                let person2 = &people_list[j];
                
                println!("\n🤝 {}与{}的互动:", person1.get_name(), person2.get_name());
                println!("   {}", person1.greet(person2));
                println!("   年龄差: {}岁", person1.age_difference(person2));
                
                if person1.is_same_city(person2) {
                    println!("   💫 他们是同城老乡！");
                }
            }
        }
    }
    
    // 测试错误处理
    println!("\n\n=== 错误处理测试 ===");
    let invalid_cases = vec![
        ("", 25, "valid@email.com", "北京", "程序员"), // 空姓名
        ("张三", 150, "valid@email.com", "北京", "程序员"), // 年龄过大
        ("李四", 25, "invalid-email", "北京", "程序员"), // 无效邮箱
        ("王五", 30, "test@example.com", "", "程序员"), // 空城市
    ];
    
    for (name, age, email, city, occupation) in invalid_cases {
        match People::new(
            name.to_string(),
            age,
            email.to_string(),
            city.to_string(),
            occupation.to_string(),
        ) {
            Ok(_) => println!("❌ 应该失败但成功了: {}, {}, {}, {}", name, age, email, city),
            Err(e) => println!("✅ 预期错误: {}", e),
        }
    }
    
    // 创建一个无职业的人物
    println!("\n\n=== 无职业人物测试 ===");
    if let Ok(person) = People::new(
        "自由职业者".to_string(),
        28,
        "freelancer@example.com".to_string(),
        "深圳".to_string(),
        "".to_string(),
    ) {
        println!("无职业人物介绍: {}", person.introduce());
        println!("完整信息:\n{}", person.get_full_info());
    }
}
