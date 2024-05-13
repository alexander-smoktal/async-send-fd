# async-send-fd

A library for sending and receiving Unix file descriptors over async UnixStream connections.

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![Build Status][actions-badge]][actions-url]

[crates-badge]: https://img.shields.io/crates/v/async-send-fd.svg
[crates-url]: https://crates.io/crates/async-send-fd
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/alexander-smoktal/async-send-fd/blob/main/LICENSE
[actions-badge]: https://github.com/alexander-smoktal/async-send-fd/actions/workflows/rust.yml/badge.svg
[actions-url]: https://github.com/alexander-smoktal/async-send-fd/actions/workflows/rust.yml

## Overview
The crate is a library for sending and receiving Unix file descriptors over [Tokio](https://crates.io/crates/tokio) or [Smol](https://crates.io/crates/smol) UnixStream connections.
You can send **RawFd** or **UnixStream** using provided interfaces.

See [test_smol_stream.rs](./tests/test_smol_stream.rs) or [test_tokio_stream.rs](./tests/test_tokio_stream.rs) for code examples.

## Creating **tokio::net::UnixStream** from **RawFd**
If you make a Tokio [UnixStream](https://docs.rs/tokio/latest/tokio/net/struct.UnixStream.html) from a raw file descriptor made by an OS call (e.g. [UnixStream::pair](https://doc.rust-lang.org/std/os/unix/net/struct.UnixStream.html)), you must make it [set_nonblocking(true)](https://doc.rust-lang.org/stable/std/os/unix/net/struct.UnixStream.html#method.set_nonblocking), otherwise receivers scheduler will block writing into the socket ⚠️

Smol [UnixStream](https://docs.rs/smol/2.0.0/smol/net/unix/struct.UnixStream.html) does it automatically if created using `UnixStream::from(Async::new(stream))`

## Transfering socket pair ownership
Sending a descriptor doesn't close the local copy, which leads to having the socket being opened by the sender until it shuts down.
If you want socket pair receivers to detect peer shutdown, you have to close local sockets after sending them.
Use [UnixStream::poll_shutdown()](https://docs.rs/tokio/latest/tokio/net/struct.UnixStream.html#method.poll_shutdown) for Tokio streams, or [UnixStream::shutdown()](https://docs.rs/smol/2.0.0/smol/net/unix/struct.UnixStream.html#method.shutdown) for Smol.

## Features
- `tokio` - for Tokio support
- `smol` - for Smol support