#![allow(unused)]
//! 词法结构: 词条
//!

/**
 ### 词条通常是宏的“类型”,用于编写各种宏代码
    ```
     // 如下宏使用了表达式作为参数类型，并且定义新的语言关键字 eval

     macro_rules! calculate {
          (eval $e:expr) => {{
               let val = $e;
               println!("{} = {}", stringify!($e), val)
          }};
     };
     calculate!(eval 1+1);
     calculate!(eval 10 * 10)
    ```
*/

pub fn main() {
    println!("")
}
