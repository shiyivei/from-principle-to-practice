use std::marker::PhantomData;

#[derive(Debug)]
struct Container<T, U> {
    data: T,
    marker: PhantomData<U>,
}

impl<T, U> Container<T, U> {
    fn new(data: T) -> Container<T, U> {
        Container {
            data,
            marker: PhantomData,
        }
    }
}

fn main() {
    // 我们知道结构体的第二个字段是u32，但是它的值是多少我们并不在意
    let _container: Container<i32, u32> = Container::new(42);
    println!("{:?}", _container)
}
