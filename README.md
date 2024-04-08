# tokio-send-fd

⚠️This crate is deprecated. Use [async-send-fd](https://crates.io/crates/async-send-fd) instead.

A library to send tokio UnixStream and raw file descriptors over tokio UnixStream connections.

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![Build Status][actions-badge]][actions-url]

[crates-badge]: https://img.shields.io/crates/v/tokio-send-fd.svg
[crates-url]: https://crates.io/crates/tokio-send-fd
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/alexander-smoktal/tokio-send-fd/blob/main/LICENSE
[actions-badge]: https://github.com/alexander-smoktal/tokio-send-fd/actions/workflows/rust.yml/badge.svg
[actions-url]: https://github.com/alexander-smoktal/tokio-send-fd/actions/workflows/rust.yml

## Overview
The crate is a library for sending and receiving Unix file descriptors over tokio UnixStream connections.
You can transfer **RawFd** or **UnixStream**. See [test_raw_fd.rs](./tests/test_raw_fd.rs) and [test_tokio_stream.rs](./tests/test_tokio_stream.rs) for examples.

## Creating **tokio::net::UnixStream** from **RawFd**
If you want to create tokio [UnixStream](https://docs.rs/tokio/latest/tokio/net/struct.UnixStream.html) from a raw file descriptor created by
os' [UnixStream::pair](https://docs.rs/tokio/latest/tokio/net/struct.UnixStream.html#method.pair) call, you should make it
[set_nonblocking(true)](https://doc.rust-lang.org/stable/std/os/unix/net/struct.UnixStream.html#method.set_nonblocking), otherwise tokio stream will block event
in async functions ⚠️

## Transfering socket pair ownership
Sending a socket of a socket pair doesn't close the local copy, which leads to having the socket being
opened until the sender is shut down.
If you want counterparties to detect peer shutdown, you have to close socket pair right after sending
a socket to a peer.
Use [close](https://docs.rs/nix/latest/nix/unistd/fn.close.html) Posix call.
