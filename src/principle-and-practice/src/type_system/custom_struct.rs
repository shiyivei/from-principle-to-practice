//! 自定义类型: 结构体
//!

/**
### 结构体种类
```
// 1.具名结构体
struct Point {
     x:f32,
     y:f32
}


// 2.元组结构体,常用于包装基本数据类型以扩展功能
struct Pair(i32,i32);
// 当元组结构体只包含一个类型是，称为NewType模式
// 如下对u32进行包装，表示分数
struct Score(u32);

impl Score {
     fn pass(&self) -> bool {
          self.0 >= 60
     }
}

let s = Score(59);
assert_eq!(s.pass(), false);

// 3.单元结构体,实例就是它自身，0大小
struct Uint;

let point = Point { x: 3.0, y: 4.0 };
let pair = Pair(1, 1);
let uint = Uint;

assert_eq!(point.x, 3.0);
assert_eq!(pair.0, 1);

```
*/

pub fn struct_type() {}

/**
### 结构体内存对齐方式

```
   // 推断结构体占12字节
    // #[repr(C)] //使用属性不让编译器自动优化布局
    struct A {
        a: u8,  // 占1字节,按照4字节对齐，补3
        b: u32, // 占4字节，补0
        c: u16, //占2字节，补2
    }

    // 实际优化,字段重排
    struct B {
        b: u32,
        c: u16,
        d: u8,
    }

    println!("{:?}", std::mem::size_of::<A>());
    println!("{:?}", std::mem::size_of::<B>());

```
*/
pub fn struct_memory_alignment() {}
