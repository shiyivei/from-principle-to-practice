//! 分号表达式和块表达式
//!
#![allow(unused)]
/**
```
;
;

{
     ()
}
{
     ();
     use std::vec::Vec;
}

();

// 块表达式的值是最后一个表达式的值,类型最后一个表达式值的类型
// 即值的的单元类型 （）or &();值的类型如 i32等
let block = &{;}; // -> &()

// 分号表达式的值都是单元类型
; // -> ()

     // 一个表表达式案例
     // 如下的循环语句都是表达式，并且每个分支的类型都是 (),在Rust中分支类型必须相同
     // 这也是为什么rust中没有问号和冒号组成的二元操作符号，rust中的逗号代表一种顺序

     for i in 1..102 {
        if i % 15 == 0 {
            println!("FizzBuzz")
        } else if i % 5 == 0 {
            println!("Fizz")
        } else if i % 3 == 0 {
            println!("Buzz")
        } else {
            println!("{}", i)
        }
    }
```
*/

pub fn semicolon_and_block_expr() {
    println!("semicolon and block expression");
}
