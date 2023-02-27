//! 基本类型: 字符串和字符
//!
/**
 ### 字符串和字符

     // 1. str
    use std::str::Chars;
    let s1: &str = "2023"; // str 的引用
    let bytes: &[u8] = "bors".as_bytes(); // 转bytes
    let s: *const u8 = "Hello".as_ptr(); // 转指针
    let word: Chars = "goodbye".chars(); // 转字符

    // 2. String
    let s2 = "2023".to_string();
    pub struct String {
        vec: Vec<u8>,
    }

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

     use std::str;
    // 将utf-8序列转换为字符串
    let tao = str::from_utf8(&[0xE9u8, 0x81u8, 0x93u8]).unwrap();
    println!("{}", tao);

    // 通过16进制码位转换为字符串
    let tao = String::from("\u{9053}");
    println!("{}", tao);
    let unicode_x = 0x9053;
    let utf_x_hex = 0xe98193;
    let utf_x_bin = 0b11101001100000011001011;

    println!("unicode_x: {:b}", unicode_x);
    println!("utf_x_hex: {:b}", utf_x_hex);
    println!("utf_x_bin: {:b}", utf_x_bin);

```
### 字符
```
    let tao = '道';

    let tao_u32 = tao as u32;
    println!("{}", tao_u32); // 字符‘道’对应的u32的值
    println!("U+{:x}", tao_u32); // 道的Unicode 字符编码
    println!("{}", tao.escape_unicode()); // 道转译后的Unicode 码点

    let a = char::from(65);
    println!("{}", a);

    //转换16进制的码点值
    if let Some(c1) = std::char::from_u32(0x9053) {
        println!("{}", c1)
    }
    if let Some(c2) = std::char::from_u32(36947) {
        println!("{}", c2)
    }

    // 并不是所有的数字都是有效的Unicode标量值
    if let Some(c3) = std::char::from_u32(129010101) {
        println!("{}", c3)
    } else {
        println!("invalid code")
    }

    use std::str;
    // 将utf-8序列转换为字符串
    let tao = str::from_utf8(&[0xE9u8, 0x81u8, 0x93u8]).unwrap();
    println!("tao:{}", tao);

    // 通过16进制码位转换为字符串
    let tao = String::from("\u{9053}");
    println!("{}", tao);
    let unicode_x = 0x9053;
    let utf_x_hex = 0xe98193;
    let utf_x_bin = 0b11101001100000011001011;

    println!("unicode_x: {:b}", unicode_x);
    println!("utf_x_hex: {:b}", utf_x_hex);
    println!("utf_x_bin: {:b}", utf_x_bin);

    // 特殊字符
    // 码位可能不同,但字节大小一样
    // 长度可能不同,但值的大小一样

    let e = 'é'; // 和 let e = 'é'; 不一样，前者是两个unicode 码点，后者是1个
                 // let e = 'é';
    let f = 'e';

    let g = "é";
    let h = "e";

    println!("e utf-8 bytes: {}", e.len_utf8()); // 占2个字节
    println!("f utf-8 bytes: {}", f.len_utf8()); // 占1个字节

    println!("e value size: {}", std::mem::size_of_val(&e)); // 4字节
    println!("f value size: {}", std::mem::size_of_val(&f)); // 4字节

    println!("g utf-8 bytes: {}", g.len()); // 2字节
    println!("h utf-8 bytes: {}", h.len()); // 1字节

    println!("g value size: {}", std::mem::size_of_val(&g)); // 16字节
    println!("h value size: {}", std::mem::size_of_val(&g)); // 16字节

    // emoji 只能是字符串
    let s = String::from("love: ❤️");
    println!("emoji {}", s)

```
*/

pub fn string_and_char() {}
