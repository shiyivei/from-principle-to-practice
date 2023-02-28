use std::{fmt::Write, io::Read, marker::PhantomData};

fn main() {
    // 一 结构体类型
    // 1.具名结构体
    struct Point {
        x: f32,
        y: f32,
    }

    // 2.元组结构体,常用于包装基本数据类型以扩展功能
    // 当元组结构体只包含一个类型是，称为NewType模式
    // 如下对u32进行包装，表示分数,可以为其扩展功能（实现trait或者方法）
    struct Score(u32);

    impl Score {
        fn pass(&self) -> bool {
            self.0 >= 60
        }
    }

    let s = Score(59);
    assert_eq!(s.pass(), false);

    // 3.单元结构体,实例就是它自身，0大小类型
    struct Uint;

    let point = Point { x: 3.0, y: 4.0 };
    let uint = Uint;

    // 结构体通过.访问字段
    assert_eq!(point.x, 3.0);

    // 二 结构体内存布局

    #[repr(C)] //该属性可以不让编译器自动优化布局，该结构体大小为 4 * 3 = 12 字节
    struct A {
        a: u8,  // 占1字节, 按照4字节对齐，补3个字节
        b: u32, // 占4字节, 不补
        c: u16, //占2字节, 补2字节
    }

    println!("{:?}", std::mem::size_of::<A>()); // 12

    // 享受编译器自动排布优化,以便CPU访问效率，按最大成员大小进行对齐
    struct B {
        a: u8,
        b: u32,
        c: u16,
    }

    // 结构体 B 实际优化时会字段重排，等价于 结构体 C
    struct C {
        b: u32,
        c: u16,
        a: u8, // 只需要 a 字段再补齐一个字节即可
    }

    println!("{:?}", std::mem::size_of::<B>()); // 8
    println!("{:?}", std::mem::size_of::<C>()); // 8

    // 三 结构体、泛型和Trait

    // 单元结构体
    #[derive(Debug)]
    pub struct Struct1;

    // 单元结构体
    #[derive(Debug)]
    pub struct Struct2;

    // 泛型结构体
    #[derive(Debug)]
    pub struct Struct3<R, T, D> {
        element_a: R,
        pub(crate) element_b: String,
        element_c: PhantomData<T>, // 使用幻影类型包裹泛型T，它允许你在定义时无需使用T，而在实现时才使用
        element_d: PhantomData<D>,
    };

    // 定义trait

    trait T1 {}
    trait T2 {}

    //
    impl<R, T> T1 for Struct3<R, T, String>
    where
        R: Read + Write, // 实现时再指定具体的类型是什么
        T: Send + Send,
    {
    }

    impl<R, T> T2 for Struct3<R, T, String>
    where
        R: Copy, // 实现时再指定具体的类型是什么
        T: AsMut<String>,
    {
    }
}
