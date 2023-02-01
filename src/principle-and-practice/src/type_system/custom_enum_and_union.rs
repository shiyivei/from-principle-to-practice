//! 自定义类型: 枚举和联合体
//!

/**
### 枚举体和联合体内存对齐方式

```
   // 枚举体实际上是带tag的联合体
    enum A {
        One,
        Two,
    }

    // 枚举体
    // 以枚举体成员最大的对齐值为准，不需要为每个枚举值都对齐
    enum E {
        N,           //判别式
        H(u32),      // 函数项构造器（类型构造器）
        M(Box<u32>), // 函数项构造器 （类型构造器）Box<u32>占八个字节，M占一个字节，需要补齐，共计 8+8=16字节
    }

    // 以联合体成员最大的对齐值为准，不需要为每个字段都对齐
    union U {
        u: u32,
        v: u64,
    }

    println!("A {:?}", std::mem::size_of::<A>()); // 1 有一个tag字段
    println!("E {:?}", std::mem::size_of::<E>()); // 16字节
    println!("U {:?}", std::mem::size_of::<U>()); // 8字节
    println!("Box<u32>: {:?}", std::mem::size_of::<Box<u32>>()) // 8
```
*/
pub fn enum_memory_alignment() {}
