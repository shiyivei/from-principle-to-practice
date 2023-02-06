//! 词法作用域和非词法作用域
//! Scope和NLL
/**
 ###
```
// 1 词法作用域
    // 一个函数的块表达式对应一个栈针 stack frame
    // 栈针的特点是函数调用完会自动清空
    // 词法作用域对应栈针
    // 基本上词法作用域等于生命周期
    let mut v = vec![];
    v.push(1);

    {
        //    println!("{:?}", v[0]);
        v.push(2);
    }
    // mir 中每一个scope都代表一个词法作用域
    /*
    scope 1 {
        debug v => _1;                   // in scope 1 at src/main.rs:3:5: 3:10
    }
    */

    // 2 非词法作用域 NLL: 案例 1

    // Rust语言编译过程
    // text -> tokens -> ast -> hir -> mir -> llvm ir -> llvm
    // 在可变借用的作用域内不允许在开辟的子生命周期中执行可变借用
    let mut v: Vec<i32> = vec![];
    let vv = &v;

    {
        //    println!("{:?}", v[0]);
        //    v.push(2); // 不允许可变借用
    }

    vv;

    // 2 非词法作用域 NLL: 案例 2
    // 替换问好

    let s = "ab?c?d";

    // 把字符串转成字符切片
    let mut chars = s.chars().collect::<Vec<char>>();

    println!("{:?}", chars);

    for i in 0..s.len() {
        // 这里不可以用可变借用
        let mut words = ('a'..'z').into_iter();
        println!("{:?}", words);

        if chars[i] == '?' {
            // 获取左边和右边的字符
            let left = if i == 0 { None } else { Some(chars[i - 1]) };
            let right = if i == s.len() - 1 {
                None
            } else {
                Some(chars[i + 1])
            };

            // 在26个字母中寻找不等于左边也不等于右边的字母进行替换
            chars[i] = words
                .find(|&w| Some(w) != left && Some(w) != right)
                .unwrap();
        }
    }
    // 将字符收集转换为字符串
    let s = chars.into_iter().collect::<String>();
    println!("{:?}", s)
```
 */

pub fn borrow_check() {}
