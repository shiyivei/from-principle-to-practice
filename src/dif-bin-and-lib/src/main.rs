use std::cell::UnsafeCell;

fn main() {
    use std::cell::RefCell;
    // 使用vec！宏创建不可变的动态可增长数组
    let vec = vec![1, 2, 3, 4];
    // vec.push(5); // 不能往不可变的数组中增加元素

    let ref_vec = RefCell::new(vec); //包裹变长数组
    println!("{:?}", ref_vec.borrow()); // 不可变借用打印
    ref_vec.borrow_mut().push(5); // 可变借用改变
    println!("{:?}", ref_vec.borrow()) // 不可变借用打印
}
