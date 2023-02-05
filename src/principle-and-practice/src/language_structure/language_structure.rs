//! Rust 函数调用栈
//!

/**

```
 * let answer = "42";
    let no_answer = answer;
    println!("{:?}", answer); //可用

    let answer = String::from("42");
    let no_answer = answer;
    // println!("{:?}", answer); //不可用，由类型系统决定

    // MIR
    // 函数调用栈
    // 运行结束时,最后一个会先被清除
    // 先进后出
    /*
    let _1: &str;
    scope 1 {
        debug answer => _1;
        let _2: &str;
        scope 2 {
            debug no_answer => _2;
            let _3: std::string::String;
            scope 3 {
                debug answer => _3;
                let _4: std::string::String;
                scope 4 {
                    debug no_answer => _4;
                }
            }
        }
    }
    */
 ```
 */

pub fn function_call_stack() {}
