fn main() {
    trait Test {
        fn foo(&self);

        fn works(self: Box<Self>) {
            println!("hello");
        }

        fn fails(self: Box<Self>)
        // where
        //     Self: Sized, //限定了被调用,关闭；？Sized 在类型声明时使用
        {
            self.foo();
        }
    }

    struct Concrete;

    impl Concrete {
        fn hello(&self) {
            println!("hello");
        }
    }

    impl Test for Concrete {
        fn foo(&self) {
            ()
        }
        fn works(self: Box<Self>) {
            self.hello();
        }
        // 没有实现fails
    }

    let concrete: Box<dyn Test> = Box::new(Concrete);
    // concrete.fails();
    concrete.works();
}
