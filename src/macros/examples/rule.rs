#[macro_export]
macro_rules! my_vec {
     // 如果没有传入任何参数，就创建一个空vec
    () => {
        std::vec::Vec::new();
    };

    // $($el:expr), *告诉编译器可以匹配任意多个以逗号分隔的表达式
    // 每个匹配到的表达式命名为$el
    ($($el:expr),*) => ({
     let mut v = std::vec::Vec::new();

     // 匹配到多少个 $el，就展开为多少个push语句
     $(v.push($el);)*

     v
    });

    //使用两个表达式构建
    ($el:expr;$n:expr) => {
     std::vec::from_elem($el,$n)
    }
}

macro_rules! handle_error {
    ($ctx:ident,$exp:expr) => {
        match $exp {
            Ok(s) => println!("{:?}", s),
            Err(e) => println!("{:?}", e),
        }
    };
}

fn main() {
    let mut v = my_vec![];
    v.push(1);

    let v1 = my_vec!(1, 2, 3, 4);

    let v2 = my_vec! {1,2,3,4};
    let v3 = my_vec![1, 2, 3, 4];

    println!("{:?},{:?},{:?}", v1, v2, v3);

    fn maybe_error(s: String) -> Result<String, std::fmt::Error> {
        Ok(s)
    }

    let s = maybe_error("rust".to_string());

    //     match s {
    //         Ok(s) => println!("{:?}", s),
    //         Err(e) => println!("{:?}", e),
    //     }

    handle_error!(s, maybe_error("rust".to_string()));

    query!(SELECT * FROM users WHERE age > 10);
}

use macros::query;
