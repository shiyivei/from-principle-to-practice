// 1. establish tcp connect
// 2. handle tcp stream: read/write

use super::*;

// RawFd 是一个表示文件描述符的原始整数类型
// RawFd 通常用于与操作系统的 I/O 相关的函数和类型中。例如，可以使用 RawFd 打开文件或管道、使用 RawFd 进行读写操作等
pub struct TcpListener(RawFd); // 听某个文件或者管道
pub struct TcpStream(pub RawFd);
pub struct Incoming<'a>(&'a TcpListener); // 循环处理

// 三个tcp中使用的对象，读、写和接收到的的 Future，它们需要实现标准库中的 Future trait
// 使用Future的概念是为了构建异步
pub struct AcceptFuture<'a>(&'a TcpListener);
pub struct ReadFuture<'a>(&'a TcpStream, &'a mut [u8]);
pub struct WriteFuture<'a>(&'a TcpStream, &'a [u8]);

// 定义 Ip地址
pub struct Ipv4Addr(libc::in_addr);

// 定义一个new方法
impl Ipv4Addr {
    pub fn new(a: u8, b: u8, c: u8, d: u8) -> Self {
        Ipv4Addr(libc::in_addr {
            s_addr: ((u32::from(a) << 24)
                | (u32::from(b) << 16)
                | (u32::from(c) << 8)
                | u32::from(d))
            .to_be(),
        })
    }
}

// 为TcpListener实现方法，比如bind等

impl TcpListener {
    // 1. 绑定地址，本质上是把地址和套接字连起来，listener就可以监听这个线路了
    pub fn bind(addr: Ipv4Addr, port: u16) -> io::Result<TcpListener> {
        let backlog = 128;

        // 生成套接字
        let sock = syscall!(socket(
            libc::PF_INET,
            libc::SOCK_STREAM | libc::O_CLOEXEC,
            libc::IPPROTO_IP
        ))?;

        let opt = 1;

        // 设置套接字
        syscall!(setsockopt(
            sock,
            libc::SOL_SOCKET,
            libc::SO_REUSEADDR,
            &opt as *const _ as *const libc::c_void,
            std::mem::size_of_val(&opt) as u32
        ))?;

        let sin: libc::sockaddr_in = libc::sockaddr_in {
            sin_family: libc::AF_INET as libc::sa_family_t,
            sin_port: port.to_be(),
            sin_addr: addr.0,
            ..unsafe { mem::zeroed() }
        };

        let addrp: *const libc::sockaddr = &sin as *const _ as *const _;
        let len = mem::size_of_val(&sin) as libc::socklen_t;
        syscall!(bind(sock, addrp, len))?;
        syscall!(listen(sock, backlog))?;

        info!("(TcpListener) listen: {}", sock);
        let listener = TcpListener(sock);
        listener.set_non_blocking()?;
        Ok(listener)
    }

    // 2. 接受内容，接收到了tcpstream,底层是系统调用接收的
    pub fn accept(&self) -> io::Result<TcpStream> {
        let mut sin_client = unsafe { mem::zeroed() };
        let addrp = &mut sin_client as *mut _ as *mut _;
        let mut len = unsafe { mem::zeroed() };
        let lenp = &mut len as *mut _;

        let socket_client = syscall!(accept(self.0, addrp, lenp))?;
        info!("(TcpStream) accept; {}", socket_client);

        Ok(TcpStream(socket_client))
    }

    // 是TcpListener的一个包装
    pub fn incoming(&self) -> Incoming<'_> {
        Incoming(self)
    }

    // 设置不阻塞，也是根据系统调用的结果来的
    pub fn set_non_blocking(&self) -> io::Result<()> {
        let flag = syscall!(fcntl(self.0, libc::F_GETFL, 0))?;
        syscall!(fcntl(self.0, libc::F_SETFL, flag | libc::O_NONBLOCK))?;
        Ok(())
    }
}

// tcpstream有读写功能，主要是对 Future携带的buf操作的，也可以设置阻塞，来源于系统调用
impl TcpStream {
    pub fn set_non_blocking(&self) -> io::Result<()> {
        let flag = syscall!(fcntl(self.0, libc::F_GETFL, 0))?;
        syscall!(fcntl(self.0, libc::F_SETFL, flag | libc::O_NONBLOCK))?;
        Ok(())
    }

    pub fn read<'a>(&'a self, buf: &'a mut [u8]) -> ReadFuture<'a> {
        ReadFuture(self, buf)
    }

    pub fn write<'a>(&'a self, buf: &'a [u8]) -> WriteFuture<'a> {
        WriteFuture(self, buf)
    }
}

// Future 非常重要，异步编程中自定义的类型必须要实现，才能实现异步
// future 类型根据poll的结果来实现异步

impl<'a> Future for AcceptFuture<'a> {
    type Output = Option<io::Result<TcpStream>>;
    // poll 函数返回两种可能：Poll::Pending 和 Poll::Ready(val),它是Future中唯一的方法
    // poll 的第一个参数是self，或者mut self,但是用Pin<&mut Self>进行了内存位置固定
    // poll 的第二个参数是一个上下文
    // poll 会一直询问等待，直到收到结果，当数据准备好的时候唤醒它
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // 里面的 TcpListener 拿到了
        match self.0.accept() {
            Ok(stream) => {
                //拿到值，设置为非阻塞
                stream.set_non_blocking()?;
                Poll::Ready(Some(Ok(stream)))
            }
            // 没拿到值返回Pending或者Err
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => Poll::Pending,
            Err(e) => Poll::Ready(Some(Err(e))),
        }
    }
}

// 读Future更加底层，需要做系统调用
impl<'a> Future for ReadFuture<'a> {
    // 定义返回值类型
    type Output = io::Result<usize>;

    // poll 函数返回两种可能：Poll::Pending 和 Poll::Ready(val),它是Future中唯一的方法
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // 使用系统调用读
        // 参数是管道和u8切片
        let res = syscall!(read(
            (self.0).0,
            self.1.as_mut_ptr() as *mut libc::c_void,
            self.1.len()
        ));

        // 根据读的结果返回
        match res {
            Ok(n) => Poll::Ready(Ok(n as usize)),
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // REACTOR.add_event((self.0).0, EpollEventType::In, cx.waker().clone())?;
                Poll::Pending
            }

            Err(e) => Poll::Ready(Err(e)),
        }
    }
}

impl<'a> Future for WriteFuture<'a> {
    type Output = io::Result<usize>;
    // poll 函数返回两种可能：Poll::Pending 和 Poll::Ready(val),它是Future中唯一的方法
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // 使用系统调用写
        // 参数是管道和u8切片
        let res = syscall!(write(
            (self.0).0,
            self.1.as_ptr() as *mut libc::c_void,
            self.1.len()
        ));
        match res {
            Ok(n) => Poll::Ready(Ok(n as usize)),
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // REACTOR.add_event((self.0).0, EpollEventType::Out, cx.waker().clone())?;
                Poll::Pending
            }
            Err(e) => Poll::Ready(Err(e)),
        }
    }
}

impl<'a> Incoming<'a> {
    // 将TcpListener返回给AcceptFuture
    pub fn next(&self) -> AcceptFuture<'a> {
        AcceptFuture(self.0)
    }
}

// 实现drop自动释放
impl Drop for TcpListener {
    fn drop(&mut self) {
        info!("TcpListener close : {}", self.0);
        syscall!(close(self.0)).ok();
    }
}
impl Drop for TcpStream {
    fn drop(&mut self) {
        info!("TcpStream close : {}", self.0);
        syscall!(close(self.0)).ok();
    }
}
