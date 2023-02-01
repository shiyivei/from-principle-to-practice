# 1 Rust语言介绍

Rust语言其他的不多强调了，但要强调一点：性能、安全以及实用是Rust追求的根本目标

## 1.1 内存安全方案

### 1.1.1 Rust针对C语言的不足

禁止对空指针和悬垂指针解引用

读取未初始化的内存

缓冲区溢出

非法释放已经释放或未分配的指针

### 1.1.2 安全无缝的沟通C语言

通过C-ABI零成本和C语言打交道

划分了Safe Rust和Unsafe Rust

# 2 Rust语言核心原理及案例

## 2.1 Rust 编译过程

![](/Users/qinjianquan/Career/rust-language/from-principle-to-practice/images/compile-process.png)

**特别说明**

大部分语言会将词条流解析到的抽象思维语法树直接转为机器码，但是rust会将其转为高级中间语言以及中级中间语言、LLVM中间语言，交由LLVM后端生成机器码

高级中间语言：类型检查、方法查找

中级中间语言：借用检查、优化、代码生成、泛型单态化等工作

版次差异在到达中级中间语言时就会消除

## 2.2 Rust词法结构

词法结构对于任何一种语言来说都非常重要，因为它不光是构成语言的必要部分，而且也关乎到语言如何解析和编译。在rust中，词法结构中的词条还涉及元编程

### 2.2.1 六大词法结构

关键字：严格关键字、保留字、弱关键字

标识符：不以数字开头的ASCII字符注释

![image-20230201085836299](/Users/qinjianquan/Career/rust-language/from-principle-to-practice/images/identifier.png)

注释：Rust可以使用注释直接生成文档，非常友好

空白：空白不表示任何含义，如换行等

词条：词条在写宏的时候非常有用

![image-20230201093310102](/Users/qinjianquan/Career/rust-language/from-principle-to-practice/images/entry.png)

![image-20230201095708674](/Users/qinjianquan/Career/rust-language/from-principle-to-practice/images/macro_use_entry.png)

路径



## 2.2 Rust 类型系统

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

![image-20230131163840335](https://github.com/shiyivei/everything-about-rust/raw/main/images/basic-data-type.png)

![image-20230131165942371](https://github.com/shiyivei/everything-about-rust/raw/main/images/integers-and-floats.png)

**特别说明**

1. usize和isize有符号和无符号指针大小类型，指针一般和计算机字长相等，32位处理器：4字节，64位处理器：8字节
2. 布尔值可以转数字，但是反过来不可以
3. 数组在Rust中是二等公民，长度不同，类型不同。等常量泛型稳定后可以晋升统一的[T;N]l 类型
4. rust中的char是unicode标量，占四个字节
5. 字符串，rust中的字符串有非常多的类型，从根本上讲是为了适应不同的场景，如下：

![image-20230131171425591](https://github.com/shiyivei/everything-about-rust/raw/main/images/string.png)

在Rust中，字符串比较复杂，涉及底层内存管理知识

![image-20230131182440796](/Users/qinjianquan/Library/Application Support/typora-user-images/image-20230131182440796.png)

6 指针类型

#### 2.2.3.2 自定义复合类型

#### 2.2.2.3 容器类型

#### 2.2.2.4 泛型

#### 2.2.2.5 特定类型

# 3 Rust核心库

# 4 Rust标准库



# 5 Rust第三方库



# 6 知名Rust项目



