//! 位置表达式和值表达式
//!

/**
 ### 位置上下文

```
  // 1. 赋值和复合赋值操作左侧
    let mut a = 1;
    a += 1;

    // 2. 一元借用和解引用操作数所在区域
    let a = &mut 7;
    *a = 42;
    // 二元借用 b:&&mut i32
    let b = &a;

    // 3.字段表达式操作数所在位置
    struct A {
        name: &'static str,
    }
    let a = A { name: "Alice" };
    a.name; //位置上下文

    // 4.数组索引表达式操作数所在区域
    let mut arr = [1, 2, 3];
    let b = &mut arr;
    arr[1];

    // 5.任意隐式借用操作数所在区域
    let mut v = vec![1, 2, 3];
    v.push(4);

    // 6.let 初始化
    let a: i32;

    // 7.if let/while let/match 的匹配表达式所在的区域
    let dish = ("ham", "eggs");
    if let ("bacon", b) = dish {
        // ("bacon",b) 就是位置上下文
        println!("bacon is served {}", b);
    } else {
        println!("No bacon will be served")
    }

    //match （位置上下文）/ while let（位置上下文） 同理

    // 结构体更新语法中的base表达式
    struct Point3d {
        x: i32,
        y: i32,
        z: i32,
    }

    let mut base = Point3d { x: 1, y: 2, z: 3 };
    let y_ref = &mut base.y;

    Point3d {
        y: 0,
        z: 10,
        ..base
    };

    // Rust 所有权语语义在表达式上的体现

    let stack_a = 42;
    let stack_b = stack_a; // 位置表达式到值上下文中，发生了copy

    stack_a;

    let heap_a = "hello".to_string();
    let heap_b = heap_a; // 位置表达式到值上下文中，发生了move

//     heap_a; error

```
 */
pub fn position_and_value_expr() {
    println!("")
}
