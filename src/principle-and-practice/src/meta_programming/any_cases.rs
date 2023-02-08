//! Any
//! 
/**
 ### 反射的案例

 ```
 // case 1 反射
    use std::any::Any;
    use std::fmt::Debug;

    fn log<T: Any + Debug>(value: &T) {
        // 将具体类型转换为 trait 对象
        let value_any = value as &dyn Any;

        //反射，判断类型，也叫自省
        match value_any.downcast_ref::<String>() {
            Some(as_string) => println!("string ({}): {}", as_string.len(), as_string),
            None => {
                println!("{:?}", value)
            }
        }
    }

    fn do_work<T: Any + Debug>(value: &T) {
        log(value)
    }

    let my_string = "hello world".to_string();
    do_work(&my_string);
    let my_i8: i8 = 100;
    do_work(&my_i8);

    // 反射如何实现
    // pub trait Any: 'static {
    //     pub fn type_id(&self) -> TypeId;
    // }

    // 为dyn Any实现了 fn is<T:Any>(&self) -> bool;方法
    // 他也是线程安全的

    // case 2

    trait Foo: Any {
        fn as_any(&self) -> &dyn Any;
    }

    impl<T: Any> Foo for T {
        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    #[derive(Debug)]
    struct Bar {}
    #[derive(Debug)]
    struct Baz {}

    impl PartialEq for dyn Foo {
        fn eq(&self, other: &dyn Foo) -> bool {
            let me = self.as_any();
            let you = other.as_any();

            if me.is::<Baz>() && you.is::<Baz>() {
                true
            } else {
                false
            }
        }
    }

    let bar = Bar {};
    let baz = Baz {};
    let foo1 = &bar;
    let foo2 = &baz;

    println!("{:?}", foo1);
    println!("{:?}", foo2);

     // Any 在第三方库中的一些应用 oso，bevy
 ```
 */