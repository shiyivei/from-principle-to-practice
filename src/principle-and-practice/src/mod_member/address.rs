//! 地址
//!
//!
/**
 ### 模块与模块调用

```
struct User {
     name:String,
     address:String
}

    pub mod a {
        pub fn a_name(s: String) -> String {
            println!("a");
            s
        }
        pub mod b {
            pub fn b_name(s: String) -> String {
                println!("b");
                s
            }
            pub mod c {
                pub fn c_name(s: String) -> String {
                    println!("c");
                    s
                }
            }
        }
    }

    a::a_name("Alice".to_string());
    a::b::b_name("Alice".to_string());
    a::b::c::c_name("Alice".to_string());


```


*/

pub fn mod_dispatch() {}
