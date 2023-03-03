use std::{
    io::{Read, Write},
    net::TcpListener,
    thread,
};

// 一个tcp server

fn main() {
    // 在传输层监听信息流
    let listener = TcpListener::bind("0.0.0.0:8000").unwrap();
    println!("Start listen on: {:?}", listener);

    loop {
        // 接收信息流
        let (mut tcp_stream, addr) = listener.accept().unwrap();
        println!("Accepted a new connection: {}", addr);

        // 使用多线程处理
        thread::spawn(move || {
            // 建立一个缓存,是一个u8数组
            let mut buf = [0u8; 12];
            // 把信息流读到缓存中
            tcp_stream.read_exact(&mut buf).unwrap();
            println!("data: {:?}", buf);
            let result = String::from_utf8_lossy(&buf);
            match result {
                std::borrow::Cow::Borrowed(value) => println!("a borrow value: {}", value),
                std::borrow::Cow::Owned(value) => println!("a owned value: {}", value),
            }

            // b 接字符串表示后面是一个字节序列
            tcp_stream.write_all(b"glad to see you!").unwrap();

            println!("data: {:?}", buf);
        });
    }
}
