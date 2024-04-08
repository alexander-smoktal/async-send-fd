use std::{
    io::{Error, ErrorKind},
    os::unix::{
        io::{AsRawFd, RawFd},
        net::UnixStream as OsUnixStream,
        prelude::{FromRawFd, IntoRawFd},
    },
};

use tokio::{io::Interest, net::UnixStream};

use passfd::FdPassingExt;

use crate::{AsyncRecvFd, AsyncSendFd};

/// A trait to send raw file descriptors
pub trait AsyncSendTokioStream {
    fn send_stream(
        &self,
        fd: UnixStream,
    ) -> impl std::future::Future<Output = Result<(), Error>> + Send;
}

/// A trait to receive raw file descriptors
pub trait AsyncRecvTokioStream {
    fn recv_stream(&self) -> impl std::future::Future<Output = Result<UnixStream, Error>> + Send;
}

impl AsyncRecvFd for UnixStream {
    async fn recv_fd(&self) -> Result<RawFd, Error> {
        loop {
            self.readable().await?;

            match self.try_io(Interest::READABLE, || self.as_raw_fd().recv_fd()) {
                Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                    continue;
                }
                r => return r,
            }
        }
    }
}

impl AsyncSendFd for UnixStream {
    async fn send_fd(&self, fd: RawFd) -> Result<(), Error> {
        loop {
            self.writable().await?;

            match self.try_io(Interest::WRITABLE, || self.as_raw_fd().send_fd(fd)) {
                Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                    continue;
                }
                r => return r,
            }
        }
    }
}

impl AsyncSendTokioStream for UnixStream {
    async fn send_stream(&self, stream: UnixStream) -> Result<(), Error> {
        let fd = stream.into_std()?.into_raw_fd();

        self.send_fd(fd).await
    }
}

impl AsyncRecvTokioStream for UnixStream {
    async fn recv_stream(&self) -> Result<UnixStream, Error> {
        let fd = self.recv_fd().await?;

        let os_stream = unsafe { OsUnixStream::from_raw_fd(fd) };
        UnixStream::from_std(os_stream)
    }
}
