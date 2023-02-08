//! 声明宏
//!
/**
 * ### 几个声明宏的案例
 *
 ```
  // 声明宏：传入两个表达式，正则匹配表达式
    // 入参
    // 替换
    macro_rules! unless {
        ($arg:expr => $branch:expr) => {
            // 自定义自己的语言
            if !$arg {
                $branch
            }
        };
    }

    fn cmp(a: i32, b: i32) {
        unless!(a > b => println!("{} < {}", a, b))
    }

    let (a, b) = (1, 3);
    cmp(a, b);

    // 用宏实现hashmap
    // * 代表一次或者多个
    macro_rules! hashmap {
        ($($key:expr => $val:expr),*) => {
            {
                let mut _map = ::std::collections::HashMap::new();
                $(
                    _map.insert($key, $val);
                )*
                _map
            }
        };
    }

    let map = hashmap! {"a" => 1,"b" => 2};
    assert_eq!(map["a"], 1);

    // 定义两套规则的宏
    // 宏的部分封装和嵌套
    macro_rules! hashmap1 {

        // 当匹配到无逗号时，继续匹配为有逗号的宏，而有逗号的宏定义好了
        ($($key:expr => $val:expr,)*) => {
            hashmap!($($key=>$val),*);
        };

        ($($key:expr => $val:expr),*) => {
            {
                let mut _map = ::std::collections::HashMap::new();
                $(
                    _map.insert($key, $val);
                )*
                _map
            }
        };
    }

    let map = hashmap1! {"a" => 1,"b" => 2,"c"=>3, };
    println!("{:?}", map);

    // 继续改进

    macro_rules! uint {
        // 单个token树,后面接一单元类型合法
        ($($x:tt)*) => {
            ()
        };
    }

    macro_rules! cap {
        // 括号内多个表达式
        ($($key:expr),*) => {
            // 数组,表达式的引用
            (<[()]>::len(&[$(uint!($key)),*])) // <[()]> 切片的类型实例
            // ((&[$(uint!($key)),*]).len())
        };
    }

    macro_rules! hashmap2 {

        // 当匹配到无逗号时，继续匹配为有逗号的宏，而有逗号的宏定义好了
        ($($key:expr => $val:expr),* $(,) *) => {
            {
                let _cap = cap!($($key),*);
                let mut _map = ::std::collections::HashMap::with_capacity(_cap);
                $(
                    _map.insert($key,$val);
                )*
                _map
            }
        };

    }

    let cap = &[(), (), ()];
    let map = hashmap2! {"a" => 1,"b" => 2,"c"=>3,"d"=>4,"e"=>5,"f"=>6 };
    println!("{:?}", map);
    // 整合多个宏
    macro_rules! hashmap3 {
        (@unit $($x:tt)*) => {
            ()
        };
        (@count $($rest:expr),*) => (<[()]>::len(&[$(hashmap3!(@unit $rest)),*]));
        ($($key:expr => $val:expr),* $(,)*) => {
            {
            let _cap = hashmap3!(@count $($key),*);

            let mut _map = ::std::collections::HashMap::with_capacity(_cap);
            $(
                _map.insert($key,$val);
            )*
            _map
        }
        };
    }

    let map = hashmap3! {
        "a"=>1,
        "b"=>2,
        "c"=>3,
    };

    println!("{:?}", map);

    // 声明宏要注意卫生性，常用来消除重复代码
}
 ```
 */
pub fn state_macro() {}
