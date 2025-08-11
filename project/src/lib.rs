// 正方形结构体
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Square {
    side_length: f64,
}

impl Square {
    // 构造函数
    pub fn new(side_length: f64) -> Result<Square, String> {
        if side_length <= 0.0 {
            Err("边长必须大于0".to_string())
        } else {
            Ok(Square { side_length })
        }
    }
    
    // 计算面积
    pub fn area(&self) -> f64 {
        self.side_length * self.side_length
    }
    
    // 计算周长
    pub fn perimeter(&self) -> f64 {
        4.0 * self.side_length
    }
    
    // 获取边长
    pub fn get_side_length(&self) -> f64 {
        self.side_length
    }
    
    // 设置边长
    pub fn set_side_length(&mut self, new_side_length: f64) -> Result<(), String> {
        if new_side_length <= 0.0 {
            Err("边长必须大于0".to_string())
        } else {
            self.side_length = new_side_length;
            Ok(())
        }
    }
    
    // 缩放正方形
    pub fn scale(&mut self, factor: f64) -> Result<(), String> {
        if factor <= 0.0 {
            Err("缩放因子必须大于0".to_string())
        } else {
            self.side_length *= factor;
            Ok(())
        }
    }
    
    // 检查是否为单位正方形（边长为1）
    pub fn is_unit_square(&self) -> bool {
        (self.side_length - 1.0).abs() < f64::EPSILON
    }
}