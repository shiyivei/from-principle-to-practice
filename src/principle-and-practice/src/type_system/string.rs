//! 基本类型: 字符串
//!
/**
 ### 字符串
     // 静态存储的字符串引用与堆上字符串的引用
```
     // 类型是 &str,字符串切片的引用,胖指针(指针和数据长度)，原属数据存放在静态存储区
    let s_static_memory = "hello";

    //不可以使用未知大小的静态存储区的原始字符串
    // let s = *s_static_memory;

    // 类型是 String,字符串的引用,智能指针(指针、容量和数据长度)，原属数据存放在堆上
    let s_heap_memory = String::from("hello");

    //不可以使用未知大小的堆上原始字符串
    // let s = *s_heap_memory;

    println!("{},{}", s_static_memory, s_heap_memory);

```
*/

pub fn string() {}
