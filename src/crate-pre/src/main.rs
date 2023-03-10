use std::fmt::Error;

fn main() {
    // 泛型数据库引擎，并对泛型进一步使用trait约束
    // 它意味着只有实现了 Operation trait 的数据库才能放入数据库引擎中
    pub struct DBEngine<T: Operation>(T);

    pub struct MySQL;
    pub struct Redis;
    pub struct MongoDB;

    pub trait Operation {
        fn insert(table: &str, key: &str, value: &str) {
            // todo!
        }
        fn del(table: &str) -> Result<(), Error> {
            // todo!

            Ok(())
        }
        fn updade(table: &str) -> Result<(), Error> {
            // todo!
            Ok(())
        }

        // 生命周期参数
        fn query<'a>(table: &'a str, key: &'a str) -> Result<&'a str, Error> {
            // todo!
            Ok("")
        }
    }

    impl Operation for MySQL {} // 实现trait
    impl Operation for Redis {} // 实现trait

    // 业务场景一
    let mysql = DBEngine(MySQL); // 可以作为DBEngine的数据库
                                 // 业务场景二
    let redis = DBEngine(Redis); // 可以作为DBEngine的数据库
                                 // 业务场景三
                                 // let mongodb = DBEngine(MongoDB); // 不可以，因为MongDB 未实现 Operation trait

    // 泛型函数
    fn foo<T>(x: T) -> T {
        x
    }

    assert_eq!(foo(1), 1);
    assert_eq!(foo("hello"), "hello");

    // 上述的函数会单态化为两个不同参数类型的函数，如下：
    fn foo_1(x: i32) -> i32 {
        x
    }
    fn foo_2(x: &'static str) -> &'static str {
        x
    }

    foo_1(1);
    foo_2("2");

    let x = 45;

    let y = 45;

    let z = x & !y;

    println!("sum: {:?}", z);
    assert_eq!(true | false, true);
}
