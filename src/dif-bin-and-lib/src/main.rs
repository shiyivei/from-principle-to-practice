fn main() {
    // 1 类型系统保证函数契约
    fn sum(a: i32, b: i32) -> i32 {
        a + b
    }

    // sum(1u32, 2u32) 违反函数契约

    // 2 断言用于防御

    fn extend_vec(v: &mut Vec<i32>, i: i32) {
        assert!(v.len() == 5);
        v.push(i)
    }

    let mut vec = vec![1, 2, 3];
    extend_vec(&mut vec, 4);
    extend_vec(&mut vec, 5);
    assert_eq!(5, vec[4]);
    extend_vec(&mut vec, 6); // panic
}
