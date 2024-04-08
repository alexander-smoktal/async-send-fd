use std::{
    io::{Error, ErrorKind},
    os::{
        fd::AsFd,
        unix::{
            io::{AsRawFd, RawFd},
            net::UnixStream as OsUnixStream,
            prelude::FromRawFd,
        },
    },
    sync::Arc,
};

use smol::{net::unix::UnixStream, Async};

use passfd::FdPassingExt;

use crate::{AsyncRecvFd, AsyncSendFd};

/// A trait to send raw file descriptors
pub trait AsyncSendSmolStream {
    fn send_stream(
        &self,
        fd: UnixStream,
    ) -> impl std::future::Future<Output = Result<(), Error>> + Send;
}

/// A trait to receive raw file descriptors
pub trait AsyncRecvSmolStream {
    fn recv_stream(&self) -> impl std::future::Future<Output = Result<UnixStream, Error>> + Send;
}

impl AsyncRecvFd for UnixStream {
    async fn recv_fd(&self) -> Result<RawFd, Error> {
        let async_io: Arc<Async<std::os::unix::net::UnixStream>> = self.clone().into();

        loop {
            async_io.readable().await?;

            match async_io.as_fd().as_raw_fd().recv_fd() {
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
        let async_io: Arc<Async<std::os::unix::net::UnixStream>> = self.clone().into();

        loop {
            async_io.writable().await?;

            match async_io.as_fd().as_raw_fd().send_fd(fd) {
                Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                    continue;
                }
                r => return r,
            }
        }
    }
}

impl AsyncSendSmolStream for UnixStream {
    async fn send_stream(&self, stream: UnixStream) -> Result<(), Error> {
        let fd = stream.as_fd().as_raw_fd();

        self.send_fd(fd).await
    }
}

impl AsyncRecvSmolStream for UnixStream {
    async fn recv_stream(&self) -> Result<UnixStream, Error> {
        let fd = self.recv_fd().await?;

        let os_stream = unsafe { OsUnixStream::from_raw_fd(fd) };
        UnixStream::try_from(os_stream)
    }
}
