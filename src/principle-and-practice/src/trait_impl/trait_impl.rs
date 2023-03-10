//! 更多 trait 实现案例
//!
//!
/**
 ### trait实现
 ```
//单个类型的解析
   let four: u32 = "4".parse().unwrap();
   println!("{}", four);

   // 元组结构体的解析
   // 解析思路是先拿到结构体中的数字，然后使用from_str转化
   use std::str::FromStr;
   #[derive(Debug, PartialEq)]
   struct Point(i32, i32);

   #[derive(Debug, PartialEq, Eq)]
   struct ParsePointError;

   // 使用trait 提供的公共的方法来解析
   // trait中有个方法是from_str,参数是字符串切片,返回值是目标类型实例
   impl FromStr for Point {
       type Err = ParsePointError;
       fn from_str(s: &str) -> Result<Self, Self::Err> {
           // 实现过程因类型而异
           let (x, y) = s
               .strip_prefix('(')
               .and_then(|s| s.strip_suffix(')'))
               .and_then(|s| s.split_once(','))
               .ok_or(ParsePointError)?;

           let x_fromstr = x.parse::<i32>().map_err(|_| ParsePointError)?;
           let y_fromstr = y.parse::<i32>().map_err(|_| ParsePointError)?;

           // Ok中包含了实例
           Ok(Point(x_fromstr, y_fromstr))
       }
   }

   let p = "(1,2)".parse::<Point>();
   assert_eq!(p.unwrap(), Point(1, 2))

   // Add trait 的定义和实现
    // 标准库中Add也是一个trait,有一个泛型默认参数是Self，指/的是实现了该trait的类型
    pub trait Add<Rhs = Self> {
        type Output;

        fn add(self, rhs: Rhs) -> Self::Output;
    }

    // 数字相加
    impl Add for u32 {
        type Output = u32;
        fn add(self, other: u32) -> u32 {
            self + other
        }
    }

    // 字符串的相加
    impl Add<&str> for String {
        type Output = String;
        fn add(mut self, other: &str) -> String {
            self.push_str(other);
            self
        }
    }

    // 在满足Trait 一致性规则（孤儿规则）的情况下可以对trait中的函数进行重载
   ```
*/
pub fn impl_trait() {}
