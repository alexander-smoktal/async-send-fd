//! **async-send-fd** is a library for sending and receiving Unix file descriptors over async UnixStream connections.
//! You can either transfer
//! - [RawFd];
//! - Tokio [UnixStream](tokio::net::UnixStream) if `tokio` feature enabled;
//! - or Smol [UnixStream](smol::net::unix::UnixStream) if `smol` feature enabled;
//!
//! ## Examles
//! See [test_raw_fd.rs](https://github.com/alexander-smoktal/async-send-fd/blob/main/tests/test_raw_fd.rs),
//! [test_smol_stream.rs](https://github.com/alexander-smoktal/async-send-fd/blob/main/tests/test_smol_fd.rs) or
//! [test_tokio_stream.rs](https://github.com/alexander-smoktal/async-send-fd/blob/main/tests/test_tokio_stream.rs) for examples.
//!
//! ## Creating a Tokio [UnixStream](tokio::net::UnixStream) from [RawFd]
//! If you make a Tokio [UnixStream](tokio::net::UnixStream) from a raw file descriptor made by an
//! OS call (e.g. [UnixStream::pair](std::os::unix::net::UnixStream::pair())), you must make it
//! [set_nonblocking(true)](std::os::unix::net::UnixStream::set_nonblocking()), otherwise receivers scheduler will block
//! writing into the socket ⚠️
//! Smol [UnixStream](smol::net::unix::UnixStream) makes it automatically if created using `UnixStream::from(Async::new(stream))`
//!
//! ## Transfering socket pair ownership
//! Sending a descriptor doesn't close the local copy, which leads to having the socket being
//! opened by the sender until it shuts down.
//! If you want socket pair receivers to detect peer shutdown, you have to close local sockets after sending them.
//! Use [close](https://docs.rs/nix/latest/nix/unistd/fn.close.html) Posix call for Tokio streams, or [UnixStream::shutdown()](smol::net::unix::UnixStream::shutdown) for Smol.
//!
//! ## Features
//! - `tokio` - for Tokio support
//! - `smol` - for Smol support
use std::{io::Error, os::unix::io::RawFd};

#[cfg(feature = "tokio")]
mod tokio_stream;
#[cfg(feature = "tokio")]
pub use tokio_stream::{AsyncRecvTokioStream, AsyncSendTokioStream};

#[cfg(feature = "smol")]
mod smol_stream;
#[cfg(feature = "smol")]
pub use smol_stream::{AsyncRecvSmolStream, AsyncSendSmolStream};

/// A trait to send raw file descriptors
pub trait AsyncSendFd {
    /// Send RawFd
    fn send_fd(&self, fd: RawFd) -> impl std::future::Future<Output = Result<(), Error>> + Send;
}

/// A trait to receive raw file descriptors
pub trait AsyncRecvFd {
    /// Receive RawFd
    fn recv_fd(&self) -> impl std::future::Future<Output = Result<RawFd, Error>> + Send;
}
