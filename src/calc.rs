use crate::common::Operation::{self};

pub struct Calc;

impl Calc {
    /// Calculate and get result
    /// 
    /// Provides addition, subtraction, multiplication, division 
    pub fn calculate(first_part: String, second_part: String, operation: Operation) -> String {
        let first: f64 = first_part.parse().unwrap();
        let second: f64 = second_part.parse().unwrap();
        match operation {
            Operation::Addition => Self::addition(first, second),
            Operation::Subtraction => Self::subtraction(first, second),
            Operation::Multiplication => Self::multiplication(first, second),
            Operation::Division => Self::division(first, second),
            _ => "".into(),
        }
    }

    pub fn calc_procent(num: String) -> String {
        let num: f64 = num.parse().unwrap();
        let result = (num * 1.0) / 100.0;
        Self::get_string_result(result)
    }

    pub fn calc_one_divide_x(num: String) -> String {
        let num: f64 = num.parse().unwrap();
        let result = 1.0 / (num);
        Self::get_string_result(result)
    }

    pub fn calc_sqr(num: String) -> String {
        let num: f64 = num.parse().unwrap();
        let result =  num.powf(2.0);
        Self::get_string_result(result)
    }

    pub fn calc_sqrt(num: String) -> String {
        let num: f64 = num.parse().unwrap();
        let result =  num.sqrt();
        Self::get_string_result(result)
    }

    fn addition(first_part: f64, second_part: f64) -> String {
        let result = first_part + second_part;
        Self::get_string_result(result)
    }
    fn subtraction(first_part: f64, second_part: f64) -> String {
        let result = first_part - second_part;
        Self::get_string_result(result)
    }
    fn multiplication(first_part: f64, second_part: f64) -> String {
        let result = first_part * second_part;
        Self::get_string_result(result)
    }
    fn division(first_part: f64, second_part: f64) -> String {
        if second_part == 0.0 {
            return "Cannot divide by zero!".to_string();
        }
        let result = first_part / second_part;
        Self::get_string_result(result)
    }

    fn get_string_result(r: f64) -> String {
        let s = format!("{:.2}", r);
        if s.ends_with(".00") {
            s[..s.len() - 3].to_string()
        } else {
            s
        }
    }
}
