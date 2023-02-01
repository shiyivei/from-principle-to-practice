//! 词法结构: 标识符
//!

/**
 ### 标识符以字母或着下划线开头
    ```
        let name = "name";
        let _100 = "number";
        let math_grade = 150;

        println!("{},{},{}",name,_100,math_grade)

    ```
*/
pub fn identifier() {
    let name = "name";
    let _100 = "number";
    let math_grade = 150;

    println!("{},{},{}", name, _100, math_grade)
}
