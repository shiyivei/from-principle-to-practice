# 1 Rust语言介绍

Rust语言其他的不多强调了，但要强调一点：性能、安全以及实用是Rust追求的根本目标

# 2 Rust语言核心原理及案例

## 2.1 Rust 类型系统

### 2.2.1 类型系统目标

保证内存安全

保证一致性

表达明确的语义

零成本抽象表达能力

### 2.2.2 Rust如何实现目标

类型：在rust中，一切皆类型

trait：trait规范了类型的行为

### 2.2.3 Rust数据类型

#### 2.2.3.1 基本数据类型

![image-20230131163840335](/Users/qinjianquan/Career/rust-language/everything-about-rust/images/basic-data-type.png)

![image-20230131165942371](/Users/qinjianquan/Career/rust-language/everything-about-rust/images/integers-and-floats.png)

**特别说明**

1. usize和isize有符号和无符号指针大小类型，指针一般和计算机字长相等，32位处理器：4字节，64位处理器：8字节
2. 布尔值可以转数字，但是反过来不可以
3. 数组在Rust中是二等公民，长度不同，类型不同。等常量泛型稳定后可以晋升统一的[T;N]l 类型
4. rust中的char是unicode标量，占四个字节
5. 字符串，rust中的字符串有非常多的类型，从根本上讲是为了适应不同的场景，如下：

![image-20230131171425591](/Users/qinjianquan/Career/rust-language/everything-about-rust/images/string.png)

#### 2.2.3.2 自定义复合类型

#### 2.2.2.3 容器类型

#### 2.2.2.4 泛型

#### 2.2.2.5 特定类型

# 3 Rust核心库



# 4 Rust标准库



# 5 Rust第三方库



# 6 知名Rust项目

