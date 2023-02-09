use libc::syscall;

use super::*;

pub struct Ipv4Addr(libc::in_addr);
pub struct TcpListener(RawFd);
pub struct TcpStream(RawFd);
pub struct Incoming<'a>(&'a TcpListener);

pub struct AcceptFuture<'a>(&'a TcpListener);
pub struct ReadFuture<'a>(&'a TcpStream, &'a mut [u8]);
pub struct WriteFuture<'a>(&'a TcpStream, &'a [u8]);

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

impl TcpListener {
    pub fn bind(addr: Ipv4Addr, port: u16) -> io::Result<TcpListener> {
        let backlog = 128;
        let sock = syscall!(socket(
            libc::PF_INET,
            libc::SOCK_STREAM | libc::SOCK_CLOEXEC,
            0
        ))?;

        let opt = 1;

        syscall!(setsocketopt(
            sock,
            libc::SOL_SOCKT,
            libc::SO_REUSEADDR,
            &opt as *const _ as *const libc::c_void,
            0 as *const _ as *const libc::v_void,
            std::mem::size_of_val(&opt) as u32
        ))?;

        let sin: libc::sockaddr_in = libc::sockaddr_in {
            sin_family: libc::AF_INET as libc::sa_family_t,
            sin_port: port.to_be(),
            sin_addr: addr.0,
            ..unsafe { mem::zeroed() }
        };

        let assrp: *const libc::sockaddr = &sin as *const _ as *const _;
        let len = mem::size_of_val(&sin) as libc::socklen_t;
        syscall!(bind(sock, addrp, len))?;
        syscall!(listen(sock, backlog))?;

        info!("(TcpListener) listen: {}", sock);
        let listner = TcpListener(sock);
        listner.setnonblocking()?;
        Ok(listner)
    }

    pub fn accept(&self) -> io::Result<TcpStream> {
        let mut sin_client = unsafe { mem::zeroed() };
        let addrp = &mut sin_client as *mut _ as *mut _;
        let mut len = unsafe { mem::zeroed() };
        let lenp = &mut len as *mut _;

        let socket_client = syscall!(accept(self.0, addrp, lenp))?;
        info!("(TcpStream) accept; {}", socket_client);

        Ok(TcpStream(socket_client))
    }

    pub fn incoming(&self) -> Incoming<'_> {
        Incoming(self)
    }

    pub fn setnonblocking(&self) -> io::Result<()> {
        let flag = syscall!(fcntl(self.0, libc::F_GETFL, 0))?;
        syscall!(fcntl(self.0, libc::F_SETFL, flag | libc::O_NONBLOCK))?;
        Ok(())
    }
}

impl<'a> Incoming<'a> {
    pub fn next(&self) -> AcceptFuture<'a> {
        AcceptFuture(self.0)
    }
}

impl<'a> Future for AcceptFuture<'a> {
    type Output = Option<io::Result<TcpStream>>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.0.accept() {
            Ok(stream) => {
                stream.setnonblocking()?;
                Poll::Ready(Some(Ok(stream)))
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => Poll::Pending,
            Err(e) => Poll::Ready(Some(Err(e))),
        }
    }
}

impl<'a> Future for ReadFuture<'a> {
    type Output = io::Result<usize>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let res = syscall!(read(
            (self.0).0,
            self.1.as_mut_ptr() as *mut libc::c_void,
            self.1.len()
        ));

        match res {
            Ok(n) => Poll::Ready(Ok(n as usize)),
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                REACTOR.add_event((self.0).0, EpollEventType::In, cx.waker().clone())?;
                Poll::Pending
            }

            Err(e) => Poll::Ready(Err(e)),
        }
    }
}
// Pin
impl<'a> Future for WriteFuture<'a> {
    type Output = io::Result<usize>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let res = syscall!(write(
            (self.0).0,
            self.1.as_ptr() as *mut libc::c_void,
            self.1.len()
        ));
        match res {
            Ok(n) => Poll::Ready(Ok(n as usize)),
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                REACTOR.add_event((self.0).0, EpollEventType::Out, cx.waker().clone())?;
                Poll::Pending
            }
            Err(e) => Poll::Ready(Err(e)),
        }
    }
}

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
