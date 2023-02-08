//! 错误处理
//!
/**
```
// 1 类型系统保证函数契约
   fn sum(a: i32, b: i32) -> i32 {
       a + b
   }

   // sum(1u32, 2u32) 违反函数契约

   // 2 断言用于防御

   fn extend_vec(v: &mut Vec<i32>, i: i32) {
       // assert!(v.len() == 5);
       v.push(i)
   }

   let mut vec = vec![1, 2, 3];
   extend_vec(&mut vec, 4);
   extend_vec(&mut vec, 5);
   // assert_eq!(5, vec[4]);
   // extend_vec(&mut vec, 6); // panic

   // 3 Option 使用map在盒内处理
   let maybe_some_string = Some(String::from("hello, world!"));
   let maybe_some_len = maybe_some_string.map(|s| s.len());
   assert_eq!(maybe_some_len, Some(13));

   fn get_longest(names: Vec<&str>) -> Option<&str> {
       if names.len() > 0 {
           let mut shortest = names[0];
           for name in names.iter() {
               if name.len() < shortest.len() {
                   shortest = *name;
               }
           }
           Some(shortest)
       } else {
           None
       }
   }

   // 使用match 黑盒处理
   fn show_shortest(names: Vec<&str>) -> &str {
       match get_longest(names) {
           Some(shortest) => shortest,
           None => "Not Found",
       }
   }

   assert_eq!(show_shortest(vec!["Uku", "Felipe"]), "Uku");
   assert_eq!(show_shortest(Vec::new()), "Not Found");

   // 返回值类型都是Option可以使用链式调用
   fn double(val: f64) -> f64 {
       val * 2.
   }

   fn square(val: f64) -> f64 {
       val.powi(2 as i32)
   }

   fn inverse(val: f64) -> f64 {
       val * -1.
   }

   fn log(val: f64) -> Option<f64> {
       match val.log2() {
           x if x.is_normal() => Some(x),
           _ => None,
       }
   }

   fn sqrt(val: f64) -> Option<f64> {
       match val.sqrt() {
           x if x.is_normal() => Some(x),
           _ => None,
       }
   }

   let number = 20.;
   let result = Option::from(number)
       .map(inverse)
       .map(double)
       .map(inverse)
       .and_then(log)
       .map(square)
       .and_then(sqrt);
   match result {
       Some(x) => println!("x was {:?}", x),
       None => println!("this failed"),
   }

   // Result
   use std::num::ParseIntError;

   type ParseResult<T> = Result<T, ParseIntError>;

   fn squares(number_str: &str) -> ParseResult<i32> {
       number_str.parse::<i32>().map(|n| n.pow(2))
   }

   match squares("10") {
       Ok(n) => assert_eq!(100, n),
       Err(err) => println!("Error: {:?}", err),
   }
```
*/
pub fn error_handle() {}
