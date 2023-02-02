fn main() {
    struct Point {
        x: i32,
        y: i32,
    }

    let (a, b) = (1, 2);

    let Point { x, y } = Point { x: 3, y: 4 };

    assert_eq!(1, a);
    assert_eq!(2, b);
    assert_eq!(3, x);
    assert_eq!(4, y);

    // 2.函数与闭包参数

    fn sum(x: String, ref y: String) -> String {
        x + y
    }

    let s = sum("1".to_owned(), "2".to_owned());
    assert_eq!(s, "12".to_owned());

    // 辅助理解 ref

    {
        let a = 42;
        let ref b = a;
        let c = &a;

        assert_eq!(b, c);

        let mut a = [1, 2, 3];
        let ref mut b = a;

        b[0] = 0;

        assert_eq!(a, [0, 2, 3])
    }

    // 3. match 表达式

    fn check_option(opt: Option<i32>) {
        match opt {
            Some(p) => println!("has value {:?}", p),
            None => println!("has no value"),
        }
    }

    /*
        fn hand_result(res: i32) -> Result<i32, dyn Error> {
            do_something(res)?;

            // 问号等价于

            match do_something(res) {
                Ok(o) => Ok(o),
                Err(e) => return SomeError(e),
            }
        }
    */

    let arr = [1, 2, 3];
    match arr {
        [1, ..] => "start with one",
        [a, b, c] => "not start with one",
    };

    let v = vec![1, 2, 3];
    match v[..] {
        [a, b] => "not match",
        [a, b, c] => "matched",
        _ => "",
    };

    // if let 表达式

    let x: &Option<i32> = &Some(3);

    // 编译器自动使用ref
    if let Some(y) = x {
        y;
    }
}
