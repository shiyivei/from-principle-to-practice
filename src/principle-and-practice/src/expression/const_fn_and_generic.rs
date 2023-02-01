//! 常量函数
//! 常量泛型

/**
 ### 常量函数

```
     // 1. 常量函数
     const fn gcd(a: u32, b: u32) -> u32 {
          match (a, b) {
               (x, 0) | (0, x) => x, //
               (x, y) if x % 2 == 0 && y % 2 == 0 => 2 * gcd(x / 2, y / 2),
               (x, y) | (y, x) if x % 2 == 0 => gcd(x / 2, y / 2),
               (x, y) if x < y => gcd((y - x) / 2, x),
               (x, y) => gcd((x - y) / 2, y),
          }
     }

     const GCD: u32 = gcd(21, 7);

     println!("{:?}", GCD);

     // 2. 元组结构体
     #[derive(Debug)]
     struct Answer(u32);
     const A: Answer = Answer(42);

     // 3. 元组
     const B:(&str,&str) = ("value1","value2");

```
*/

/// 常量函数
pub fn const_fn() {
    println!("const_fn")
}

/**

### 常量泛型
```

use core::mem::MaybeUninit;

#[derive(Debug)]
pub struct ArrayVec<T, const N: usize> {
    items: [MaybeUninit<T>; N],
    length: usize,
}

fn main() {
    println!();

    let av = ArrayVec {
        items: [MaybeUninit::<u32>::uninit(); 3],
        length: 10,
    };

    println!("{:#?}", av);



    // array_chunks 方法是基于常量泛型对数组进行分割处理

    let data = [1, 2, 3, 4, 5, 6];
    let sum1 = data.array_chunks().map(|&[x, y]| x * y).sum::<i32>();
    let sum2 = data.array_chunks().map(|&[x, y, z]| x * y * z).sum::<i32>();
    assert_eq!(sum1, (1 * 2) + (3 * 4) + (5 * 6));
    assert_eq!(sum2, (1 * 2 * 3) + (4 * 5 * 6));

    println!("{},{}", sum1, sum2);
}


```
*/

/// 常量泛型
pub fn const_generics() {
    println!("const_generics")
}
