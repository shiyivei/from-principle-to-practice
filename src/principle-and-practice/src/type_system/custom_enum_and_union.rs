//! 自定义类型: 枚举和联合体
//!

/**
### 枚举体和联合体内存对齐方式

```

 // 枚举描述了一种情况下的所有可能
    // 如IP地址种类可能包含IPV6和IPV4
    // 成员为单元类型的枚举从它创建的那一刻开始就已经确定了
    enum IpAddrKind {
        V4,
        V6,
    }

    // 需要在使用时再赋值的枚举
    enum SubjectScore {
        Math(u32),
        English(u32),
    }

    // 包含多种类型的枚举
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    // 枚举值的访问

    let v4 = IpAddrKind::V4;
    let v6 = IpAddrKind::V6;

    let math_score = SubjectScore::Math(98);
    let english_score = SubjectScore::English(97);

    // 枚举体的内存布局
    // 以最大成员对齐值为准,枚举的大小等于最大成员的大小 + tag 对齐后的大小(64位系统下最大为8字节)
    // 枚举的本质：实际上是带tag的联合体，tag可以理解为成员的索引

    // 1+7 + 8 = 16
    enum Type {
        T,
        U(u32),
        B(Box<u32>), // 8
    }

    enum E {
        X, // 也称判别式，0大小
        Y,
    }

    println!("The size of Type : {:?}", std::mem::size_of::<Type>()); // 16
    println!("The size of E : {:?}", std::mem::size_of::<E>()); // 1
    println!("The size of u32 : {:?}", std::mem::size_of::<u32>()); // 4
    println!(
        "The size of Box<u32> : {:?}",
        std::mem::size_of::<Box<u32>>()
    ); // 8 usize

    // 联合体不带tag，大小为最大成员大小
    union U {
        u: u32, // 4
        v: u64, // 8
    }

    println!("The size of U : {:?}", std::mem::size_of::<U>()); // 8

    // 枚举体的布局优化:Option

    println!(
        "The size of Result<u8,E> : {:?}",
        std::mem::size_of::<Result<u8, std::io::Error>>()
    );
    println!(
        "The size of Result<f64,E> : {:?}",
        std::mem::size_of::<Result<f64, std::io::Error>>()
    );
    println!(
        "The size of Result<&u8,E> : {:?}",
        std::mem::size_of::<Result<&u8, std::io::Error>>()
    );
    println!(
        "The size of Result<Box<u8>,E> : {:?}",
        std::mem::size_of::<Result<Box<u8>, std::io::Error>>()
    );
    println!(
        "The size of Result<&[u8],E> : {:?}",
        std::mem::size_of::<Result<&[u8], std::io::Error>>()
    );
    println!(
        "The size of Result<String,E> : {:?}",
        std::mem::size_of::<Result<String, std::io::Error>>()
    );
    println!(
        "The size of Result<<Vec<u8>,E> : {:?}",
        std::mem::size_of::<Result<Vec<u8>, std::io::Error>>()
    );
    println!(
        "The size of Result<HashMap<String,String>,E> : {:?}",
        std::mem::size_of::<Result<HashMap<String, String>, std::io::Error>>()
    );

    println!("The size of f64 : {:?}", std::mem::size_of::<f64>());
    println!(
        "The size of Option : {:?}",
        std::mem::size_of::<Option<u8>>()
    );
    println!(
        "The size of Option : {:?}",
        std::mem::size_of::<Option<f64>>()
    );
    println!(
        "The size of Option : {:?}",
        std::mem::size_of::<Option<&u8>>()
    );
    println!(
        "The size of Option : {:?}",
        std::mem::size_of::<Option<Box<u8>>>()
    );
    println!(
        "The size of Option : {:?}",
        std::mem::size_of::<Option<&[u8]>>()
    );

    println!(
        "The size of Option : {:?}",
        std::mem::size_of::<Option<String>>()
    );
    println!(
        "The size of Option : {:?}",
        std::mem::size_of::<Option<Vec<u8>>>()
    );
    println!(
        "The size of Option : {:?}",
        std::mem::size_of::<Option<HashMap<String, String>>>()
    );

    // 枚举体的优化：Result
    // 三个字段的枚举没有优化tag
    enum R {
        O(String),
        L(),
        N(),
        //   ..
    }

    println!("The size of R : {:?}", std::mem::size_of::<R>()); // 32

    // 只有两个字段的类 Result 枚举优化了 tag
    enum S {
        O(String),

        L(),
    }

    println!("The size of S : {:?}", std::mem::size_of::<S>()); // 24

    // 非引用的不优化
    enum M {
        O(u32),
        L(),
        N(),
        //   ..
    }

    println!("The size of M : {:?}", std::mem::size_of::<M>()); // 8

    enum N {
        O(u32),
        L(),
    }

    println!("The size of N : {:?}", std::mem::size_of::<N>()); // 8

    // 应用场景

    // 在这之前一个特殊的枚举类型，在这里不做讲解，图个新鲜，挖坑后补
    pub enum Cow<'a, B: ?Sized + 'a>
    where
        B: ToOwned, // early bound
    {
        // 借用的数据
        Borrowed(&'a B),
        // 拥有的数据
        Owned(<B as ToOwned>::Owned), // 在rust中，子类型可以强强制转换为父类型
    }

    struct LinearKnob {
        position: f64,
    }

    enum EnumFunction {
        Judge,
        Sum(i32, i32),
        NormalFunction(LinearKnob),
    }

    // 场景一: 作为判别式

    if let judge = EnumFunction::Judge {
        println!("got  the right value")
    }

    // 场景二：等同于函数项构造器、元组结构体构造器

    struct A;

    impl A {
        fn sum(a: i32, b: i32) -> i32 {
            a + b
        }
    }

    // 函数项
    let fn_item = A::sum;

    fn_item(10, 10);

    let enum_fn_item = EnumFunction::Sum(10, 10);

    if let EnumFunction::Sum(a, b) = enum_fn_item {
        let sum = a + b;
    }

    // 场景三：抽象类型

    let linear_knob = EnumFunction::NormalFunction(LinearKnob { position: 42.0 });


```
*/
pub fn enum_memory_alignment() {}
