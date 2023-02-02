//! 函数及函数项
//!
/**
### 函数及函数项
### 函数
```
// 1.自由函数
fn freedom_function(des: &str) -> Option<&str> {
    Some(des)
}

struct A(i32, i32);
impl A {
    // 2.关联函数
    fn sum(x: i32, y: i32) -> i32 {
        x + y
    }

    // 3.方法
    fn math(&self) -> i32 {
        Self::sum(self.0, self.1) // 关联函数调用使用比目鱼符号
    }

    // 关联函数
    fn function_item(x: i32) -> i32 {
        x
    }
}

// 自由函数调用
freedom_function("hello");
let a = A(1,2);
// 方法调用,点号
let x = a.math();

// 关联函数调用,::号
let y = A::sum(1, 3);

```

### 函数项

```
struct A(i32, i32);
impl A {
    // 2.关联函数
    fn sum(x: i32, y: i32) -> i32 {
        x + y
    }

    // 3.方法
    fn math(&self) -> i32 {
        Self::sum(self.0, self.1) // 关联函数调用使用比目鱼符号
    }

    // 关联函数
    fn function_item(x: i32) -> i32 {
        x
    }
}

let a = A(1,2);
let x = a.math();
let y = A::sum(1, 3);

// 1.函数项构造: 类型::函数/方法名构建函数项以及自由函数的直接赋值
// 2.函数项类型: 如 fn sum(i32,i32)-> i32,就是函数签名，同trait中的方法签名一样
let add = A::sum; //Fn item 类型
let add_math = A::math; // Fn item 类型

// 3.函数项的使用: 作为函数调用
assert_eq!(add(1, 2), A::sum(1, 2));
assert_eq!(add_math(&a), a.math());

println!("{}", x);

// 4.函数项类型本质: 0大小类型,会在类型中记录函数信息
//  好处:优化函数调用

// 5.同函数项类型一样的其他类型构造器:枚举体和单元结构体

// 5.1 函数项类型
let fn_item = A::function_item;

// 等价于
// fn function_item(_1:i32)->i32 {/* */}

// 5.1 枚举体
enum Color {
     R(i32),
     G(i32),
     B(i32),
}
// 等价于
// fn Color::R(_1: i32) -> Color {/* */}
// fn Color::G(_1: i32) -> Color {/* */}
// fn Color::B(_1: i32) -> Color {/* */}

// 5.单元结构体
struct UintStruct(i32, i32);

// 等价于
// fn UintStruct(_1: i32,_2: i32) -> UintStruct {/* */}

// 6 函数项默认实现的 trait
// Copy/Clone/Send/Sync/Fn/FnMut/FnOnce

// 7 函数项可以作为函数参数（函数项可以当做变量）:函数项隐式转换为函数指针

// 定义一个类型别名作为返回值的类型（RGB是三元组的类型别名）
type RGB = (i16, i16, i16);

// 自由函数
fn color(s: &str) -> RGB {
     (1, 1, 1)
}

// 参数类型是函数指针类型的自由函数
fn show(c: fn(&str) -> RGB) {
     println!("{:?}", c("black"))
}

// 将函数变为函数项
let rgb = color;
// 将函数项显式转换为函数指针
let c: fn(&str) -> RGB = rgb;

// 函数指针作为另一个函数参数
show(c);
// 函数指针作为另一个函数参数,隐式转换
show(rgb);

println!("the size of fn item {:?}", std::mem::size_of_val(&rgb)); // 0
println!("the size of fn pointer {:?}", std::mem::size_of_val(&c)); // 8

// 8 结论
// 8.1 函数项类型可以显式转换为函数指针类型
// 8.2 尽量使用函数项类型而不是函数指针,以享受零大小类型的优化（直接用，不要作为参数传递）

```
*/
pub fn function() {}
