use std::mem::take;

fn main() {
    let x = Box::new(42);
    // 通过解引用来获取所包裹的值
    let y = *x;

    assert_eq!(y, 42);

    // 标准库中的drop实现，编译器的行为
    /*
     unsafe impl<#[may_dangle] T: ?Sized> Drop for Box<T> {
        fn drop(&mut self) {
             FIXME:Do nothing,drop is currently performed by compiler
        }
    }
    */

    // 定义自己的智能指针: 让指针可以自动解引用
    use std::ops::Deref;
    // 定义一个泛型单元组结构体
    struct MySP<T>(T);

    // 实现一个new方法
    impl<T> MySP<T> {
        fn new(s: T) -> MySP<T> {
            MySP(s)
        }
    }
    // 为类型实现Deref trait 现在类型MySP就是一个自定义的智能指针了
    impl<T> Deref for MySP<T> {
        type Target = T;
        fn deref(&self) -> &T {
            &self.0
        }
    }

    // 调用
    let x = 5;
    let y = MySP::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // 1. 自动解引用 点调用操作
    // 自定义一个类型
    #[derive(Copy, Clone)]
    struct User {
        name: &'static str,
    }

    impl User {
        fn name(&self) {
            println!("{:?}", self.name);
        }
    }

    // 调用

    let u = User { name: "Alex" };
    // 原来的调用方式

    println!("{}", u.name);
    // 使用自定义的智能指针包裹
    let y = MySP::new(u);

    // 包裹后的调用方式
    // 这里智能指针实际上自动进行了解引用,获取了里面的值，然后用值进行关联函数调用

    println!("{}", y.name);
    // 手动解引用
    let z = *y;

    println!("{}", z.name);

    // 结论：使用类型直接调用字段 = 智能指针解引用调用 = 手动解引用调用

    // 2. 自动解引用 函数参数
    fn takes_str(s: &str) {
        println!("{}", s);
    }

    let s = String::from("hello");
    // String 也是一个智能指针，它包裹了 str
    // 自动解引用为原始类型str后要再加&

    // 调用
    takes_str(&s);

    // 标准库中为String类型实现了Deref trait
    /*
    impl ops::Deref for String {
        type Target = str;

        #[inline]
        fn deref(&self) -> &str {
            unsafe { str::from_utf8_unchecked(&self.vec) }
        }
    }
    */

    // 自动解引用需要注意的地方
    // 使用*x 解引用等价于 *（x.deref)

    let s = Box::new("world");
    let ref_s1 = *s;
    let ref_s2 = *(s.deref());

    assert_eq!(ref_s1, ref_s2);

    // 自动解引用等价于 x.deref()

    // 标准库中给泛型T实现的 Deref trait
    /*
    impl<T: ?Sized> const Deref for &T {
        type Target = T;

        fn deref(&self) -> &T {
            *self
        }
    }

    impl<T: ?Sized> !DerefMut for &T {}

    ** 在日常开发中非常实用
    ** 当我们拥有可变引用T时如果还想使用T,则可以自动解引用，比如点调用
    impl<T: ?Sized> const Deref for &mut T {
        type Target = T;

        fn deref(&self) -> &T {
            *self
        }
    }
    */
}
