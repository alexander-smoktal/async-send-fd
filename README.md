# async-send-fd

A library for sending and receiving Unix file descriptors over async UnixStream connections.

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![Build Status][actions-badge]][actions-url]

[crates-badge]: https://img.shields.io/crates/v/tokio-send-fd.svg
[crates-url]: https://crates.io/crates/async-send-fd
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/alexander-smoktal/async-send-fd/blob/main/LICENSE
[actions-badge]: https://github.com/alexander-smoktal/async-send-fd/actions/workflows/rust.yml/badge.svg
[actions-url]: https://github.com/alexander-smoktal/async-send-fd/actions/workflows/rust.yml

## Overview
The crate is a library for sending and receiving Unix file descriptors over `tokio` or `smol` UnixStream connections.
You can transfer **RawFd** or **UnixStream**. See [test_raw_fd.rs](./tests/test_raw_fd.rs), [test_smol_stream.rs](./tests/test_smol_stream.rs), or [test_tokio_stream.rs](./tests/test_tokio_stream.rs) for examples.

## Creating **tokio::net::UnixStream** from **RawFd**
If you make a tokio [UnixStream](tokio::net::UnixStream) from a raw file descriptor made by an OS call (e.g. [UnixStream::pair](std::os::unix::net::UnixStream::pair())), you must make it [set_nonblocking(true)](std::os::unix::net::UnixStream::set_nonblocking()), otherwise receiver schedulers will block writing into the socket ⚠️
Smol [UnixStream](smol::net::unix::UnixStream) makes it automatically if created using `UnixStream::from(Async::new(stream))`

## Transfering socket pair ownership
Sending a descriptor doesn't close the local copy, which leads to having the socket being opened by the sender until it shuts down.
If you want socket pair receivers to detect peer shutdown, you have to close local sockets after sending them.
Use [close](https://docs.rs/nix/latest/nix/unistd/fn.close.html) Posix call for tokio streams, or [UnixStream::shutdown()](smol::net::unix::UnixStream::shutdown) for `smol`.
