//! 迭代器
//!
/**
 ###
```
  // 迭代器trait
    trait Iterator {
        type Item;

        fn next(&mut self) -> Option<Self::Item>;
    }

    // 外部迭代器语法糖for循环，相当于迭代器的next方法
    // Vec实现了迭代器trait
    let v = vec![1, 2, 3, 4, 5];
    {
        // 使用into_iter方法获得迭代器
        let mut _iterator = v.into_iter();
        loop {
            // match 匹配每一次的迭代结果
            match _iterator.next() {
                Some(i) => {
                    println!("{}", i);
                }
                None => break,
            }
        }
    }

    // 使用for循环遍历
    let v = vec![1, 2, 3, 4, 5];
    for i in 0..v.len() {
        println!("{}", v[i]);
    }

    // 自定义的内部迭代器（不是主要的模式）
    trait InIterator<T: Copy> {
        // 指定约束是为了把闭包作为参数传递
        fn each<F: Fn(T) -> T>(&mut self, f: F);
    }

    impl<T: Copy> InIterator<T> for Vec<T> {
        fn each<F: Fn(T) -> T>(&mut self, f: F) {
            let mut i = 0;
            while i < self.len() {
                self[i] = f(self[i]);
                i += 1;
            }

            // 等价于
            // for i in 0..self.len() {
            //     self[i] = f(self[i]);
            // }
        }
    }

    let mut v = vec![1, 2, 3];
    v.each(|i| i * 3);
    assert_eq!([3, 6, 9], &v[..3])

    // laziness
    let v = vec![1, 2, 3, 4, 5];
    // 未迭代
    // 边迭代,边使用map处理
    v.iter().map(|x| println!("{x}"));

    let num = 1..10;
    // let three = num.take(3); // 迭代三个
    // 收集时开始迭代
    let three: Vec<i32> = num.take(3).collect(); // 迭代三个

    // 迭代器适配器有很多

```
*/
pub fn iterator() {}
