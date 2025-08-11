// People结构体
#[derive(Debug, Clone, PartialEq)]
pub struct People {
    name: String,
    age: u32,
    email: String,
    city: String,
    occupation: String,
}

impl People {
    // 构造函数
    pub fn new(name: String, age: u32, email: String, city: String, occupation: String) -> Result<People, String> {
        if name.trim().is_empty() {
            return Err("姓名不能为空".to_string());
        }
        if age > 120 {
            return Err("年龄不能超过120岁".to_string());
        }
        if !email.contains('@') || !email.contains('.') {
            return Err("邮箱格式不正确，需要包含@和.".to_string());
        }
        if city.trim().is_empty() {
            return Err("城市不能为空".to_string());
        }
        
        Ok(People {
            name: name.trim().to_string(),
            age,
            email: email.trim().to_string(),
            city: city.trim().to_string(),
            occupation: occupation.trim().to_string(),
        })
    }
    
    // 获取姓名
    pub fn get_name(&self) -> &str {
        &self.name
    }
    
    // 获取年龄
    pub fn get_age(&self) -> u32 {
        self.age
    }
    
    // 获取邮箱
    pub fn get_email(&self) -> &str {
        &self.email
    }
    
    // 获取城市
    pub fn get_city(&self) -> &str {
        &self.city
    }
    
    // 获取职业
    pub fn get_occupation(&self) -> &str {
        &self.occupation
    }
    
    // 设置年龄
    pub fn set_age(&mut self, new_age: u32) -> Result<(), String> {
        if new_age > 120 {
            Err("年龄不能超过120岁".to_string())
        } else {
            self.age = new_age;
            Ok(())
        }
    }
    
    // 设置城市
    pub fn set_city(&mut self, new_city: String) -> Result<(), String> {
        if new_city.trim().is_empty() {
            Err("城市不能为空".to_string())
        } else {
            self.city = new_city.trim().to_string();
            Ok(())
        }
    }
    
    // 设置职业
    pub fn set_occupation(&mut self, new_occupation: String) {
        self.occupation = new_occupation.trim().to_string();
    }
    
    // 更新邮箱
    pub fn set_email(&mut self, new_email: String) -> Result<(), String> {
        if !new_email.contains('@') || !new_email.contains('.') {
            Err("邮箱格式不正确，需要包含@和.".to_string())
        } else {
            self.email = new_email.trim().to_string();
            Ok(())
        }
    }
    
    // 自我介绍
    pub fn introduce(&self) -> String {
        if self.occupation.is_empty() {
            format!("你好，我是{}，今年{}岁，住在{}，联系邮箱：{}", 
                    self.name, self.age, self.city, self.email)
        } else {
            format!("你好，我是{}，今年{}岁，职业是{}，住在{}，联系邮箱：{}", 
                    self.name, self.age, self.occupation, self.city, self.email)
        }
    }
    
    // 庆祝生日
    pub fn celebrate_birthday(&mut self) -> String {
        self.age += 1;
        format!("🎉 {}生日快乐！现在{}岁了！🎂", self.name, self.age)
    }
    
    // 判断是否成年
    pub fn is_adult(&self) -> bool {
        self.age >= 18
    }
    
    // 判断是否老年人
    pub fn is_senior(&self) -> bool {
        self.age >= 60
    }
    
    // 获取年龄组
    pub fn get_age_group(&self) -> &str {
        match self.age {
            0..=2 => "婴儿",
            3..=5 => "幼儿",
            6..=12 => "儿童",
            13..=17 => "青少年", 
            18..=35 => "青年",
            36..=59 => "中年人",
            60..=79 => "老年人",
            _ => "高龄老人",
        }
    }
    
    // 计算到退休的年数（假设65岁退休）
    pub fn years_to_retirement(&self) -> i32 {
        65 - self.age as i32
    }
    
    // 判断是否同城
    pub fn is_same_city(&self, other: &People) -> bool {
        self.city.to_lowercase() == other.city.to_lowercase()
    }
    
    // 年龄差
    pub fn age_difference(&self, other: &People) -> u32 {
        if self.age > other.age {
            self.age - other.age
        } else {
            other.age - self.age
        }
    }
    
    // 获取完整信息
    pub fn get_full_info(&self) -> String {
        format!("姓名：{}\n年龄：{}岁\n职业：{}\n城市：{}\n邮箱：{}\n年龄组：{}\n成年状态：{}\n距离退休：{}年",
                self.name,
                self.age,
                if self.occupation.is_empty() { "未设置" } else { &self.occupation },
                self.city,
                self.email,
                self.get_age_group(),
                if self.is_adult() { "已成年" } else { "未成年" },
                self.years_to_retirement())
    }
    
    // 发送问候
    pub fn greet(&self, other: &People) -> String {
        if self.is_same_city(other) {
            format!("{}对{}说：\"你好，老乡！我们都在{}呢！\"", self.name, other.name, self.city)
        } else {
            format!("{}对{}说：\"你好！我来自{}，你来自{}\"", self.name, other.name, self.city, other.city)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_people_creation_success() {
        let person = People::new(
            "张三".to_string(),
            25,
            "zhangsan@example.com".to_string(),
            "北京".to_string(),
            "程序员".to_string()
        ).unwrap();
        
        assert_eq!(person.get_name(), "张三");
        assert_eq!(person.get_age(), 25);
        assert_eq!(person.get_occupation(), "程序员");
    }

    #[test]
    fn test_people_creation_failure() {
        // 空姓名
        assert!(People::new("".to_string(), 25, "test@example.com".to_string(), "北京".to_string(), "程序员".to_string()).is_err());
        
        // 年龄过大
        assert!(People::new("张三".to_string(), 150, "test@example.com".to_string(), "北京".to_string(), "程序员".to_string()).is_err());
        
        // 邮箱格式错误
        assert!(People::new("张三".to_string(), 25, "invalid-email".to_string(), "北京".to_string(), "程序员".to_string()).is_err());
        
        // 城市为空
        assert!(People::new("张三".to_string(), 25, "test@example.com".to_string(), "".to_string(), "程序员".to_string()).is_err());
    }

    #[test]
    fn test_birthday_celebration() {
        let mut person = People::new(
            "李四".to_string(),
            24,
            "lisi@example.com".to_string(),
            "上海".to_string(),
            "设计师".to_string()
        ).unwrap();
        
        let birthday_msg = person.celebrate_birthday();
        assert_eq!(person.get_age(), 25);
        assert!(birthday_msg.contains("李四"));
        assert!(birthday_msg.contains("25岁"));
    }

    #[test]
    fn test_age_classification() {
        let child = People::new("小明".to_string(), 10, "xiaoming@example.com".to_string(), "广州".to_string(), "学生".to_string()).unwrap();
        let adult = People::new("王五".to_string(), 30, "wangwu@example.com".to_string(), "深圳".to_string(), "医生".to_string()).unwrap();
        let senior = People::new("赵六".to_string(), 70, "zhaoliu@example.com".to_string(), "杭州".to_string(), "退休".to_string()).unwrap();
        
        assert!(!child.is_adult());
        assert_eq!(child.get_age_group(), "儿童");
        
        assert!(adult.is_adult());
        assert!(!adult.is_senior());
        assert_eq!(adult.get_age_group(), "青年");
        
        assert!(senior.is_senior());
        assert_eq!(senior.get_age_group(), "老年人");
    }

    #[test]
    fn test_people_interaction() {
        let person1 = People::new("张三".to_string(), 25, "zhangsan@example.com".to_string(), "北京".to_string(), "程序员".to_string()).unwrap();
        let person2 = People::new("李四".to_string(), 30, "lisi@example.com".to_string(), "北京".to_string(), "设计师".to_string()).unwrap();
        let person3 = People::new("王五".to_string(), 28, "wangwu@example.com".to_string(), "上海".to_string(), "教师".to_string()).unwrap();
        
        // 同城测试
        assert!(person1.is_same_city(&person2));
        assert!(!person1.is_same_city(&person3));
        
        // 年龄差测试
        assert_eq!(person1.age_difference(&person2), 5);
        assert_eq!(person1.age_difference(&person3), 3);
        
        // 问候测试
        let greeting1 = person1.greet(&person2);
        let greeting2 = person1.greet(&person3);
        
        assert!(greeting1.contains("老乡"));
        assert!(!greeting2.contains("老乡"));
    }
}