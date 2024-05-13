use std::{
    io::{Error, ErrorKind},
    os::unix::{
        io::{AsRawFd, RawFd},
        net::UnixStream as OsUnixStream,
        prelude::{FromRawFd, IntoRawFd},
    },
};

use tokio::{
    io::Interest,
    net::{
        unix::{OwnedReadHalf, OwnedWriteHalf, ReadHalf, WriteHalf},
        UnixStream,
    },
};

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

impl AsyncRecvFd for ReadHalf<'_> {
    async fn recv_fd(&self) -> Result<RawFd, Error> {
        self.as_ref().recv_fd().await
    }
}

impl AsyncRecvTokioStream for ReadHalf<'_> {
    async fn recv_stream(&self) -> Result<UnixStream, Error> {
        self.as_ref().recv_stream().await
    }
}

impl AsyncSendFd for WriteHalf<'_> {
    async fn send_fd(&self, fd: RawFd) -> Result<(), Error> {
        self.as_ref().send_fd(fd).await
    }
}

impl AsyncSendTokioStream for WriteHalf<'_> {
    async fn send_stream(&self, stream: UnixStream) -> Result<(), Error> {
        self.as_ref().send_stream(stream).await
    }
}

impl AsyncRecvFd for OwnedReadHalf {
    async fn recv_fd(&self) -> Result<RawFd, Error> {
        self.as_ref().recv_fd().await
    }
}

impl AsyncRecvTokioStream for OwnedReadHalf {
    async fn recv_stream(&self) -> Result<UnixStream, Error> {
        self.as_ref().recv_stream().await
    }
}

impl AsyncSendFd for OwnedWriteHalf {
    async fn send_fd(&self, fd: RawFd) -> Result<(), Error> {
        self.as_ref().send_fd(fd).await
    }
}

impl AsyncSendTokioStream for OwnedWriteHalf {
    async fn send_stream(&self, stream: UnixStream) -> Result<(), Error> {
        self.as_ref().send_stream(stream).await
    }
}
