use std::os::unix::{io::AsRawFd, net::UnixStream as OsUnixStream, prelude::FromRawFd};

use tempdir::TempDir;

use smol::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::unix::{UnixListener, UnixStream},
    Async,
};

use async_send_fd::{AsyncRecvFd, AsyncRecvSmolStream, AsyncSendFd, AsyncSendSmolStream};

const SOCKET_NAME: &str = "smol_send_fd_test.sock";

#[test]
fn send_raw_fd_test() {
    let tmp_dir = TempDir::new("tokio-send-fd").unwrap();

    let sock_path = tmp_dir.path().join(SOCKET_NAME);
    let sock_path1 = sock_path.clone();
    let sock_path2 = sock_path.clone();

    println!("Start listening at: {:?}", sock_path1);
    let listener = UnixListener::bind(sock_path1).unwrap();

    smol::block_on(async {
        let j1 = smol::spawn(async move {
            let (stream, _) = listener.accept().await.unwrap();

            println!("Incoming peer connection");
            let (left, right) = OsUnixStream::pair().unwrap();

            println!("Sending peer fd");
            stream.send_fd(left.as_raw_fd()).await.unwrap();
            println!("Succesfullt sent peer fd");

            right.set_nonblocking(true).unwrap();
            let mut peer_stream = UnixStream::from(Async::new(right).unwrap());
            let mut buffer = [0u8; 4];

            println!("Reading data from the peer");
            assert!(peer_stream.read(&mut buffer).await.unwrap() == 4);

            println!("Message sent through a socket: {:?}", buffer);
        });

        let j2 = smol::spawn(async move {
            println!("Connection to the sender");
            let stream = UnixStream::connect(sock_path2).await.unwrap();

            println!("Succesfully connected to the sender. Reading file descriptor");
            let fd = stream.recv_fd().await.unwrap();
            println!("Succesfully read file descriptor");

            let os_stream = unsafe { OsUnixStream::from_raw_fd(fd) };

            let mut peer_stream = UnixStream::from(Async::new(os_stream).unwrap());

            println!("Sending data to the peer");
            let buffer: [u8; 4] = [0, 0, 0, 42];
            peer_stream.write(&buffer).await.unwrap();
            println!("Succesfully sent data to the peer");
        });

        smol::future::zip(j1, j2).await;
    });

    let _ = std::fs::remove_dir(sock_path);
}

#[test]
fn send_tokio_stream_test() {
    let tmp_dir = TempDir::new("smol-send-fd").unwrap();

    let sock_path = tmp_dir.path().join(SOCKET_NAME);
    let sock_path1 = sock_path.clone();
    let sock_path2 = sock_path.clone();
    smol::block_on(async {
        println!("Start listening at: {:?}", sock_path1);
        let listener = UnixListener::bind(sock_path1).unwrap();

        let j1 = smol::spawn(async move {
            let (stream, _) = listener.accept().await.unwrap();

            println!("Incoming peer connection");
            let (left, mut right) = UnixStream::pair().unwrap();

            println!("Sending peer fd");
            stream.send_stream(left).await.unwrap();
            println!("Succesfullt sent peer fd");

            let mut buffer = [0u8; 4];

            println!("Reading data from the peer");
            assert!(right.read(&mut buffer).await.unwrap() == 4);

            println!("Message sent through a socket: {:?}", buffer);
        });

        let j2 = smol::spawn(async move {
            println!("Connection to the sender");
            let stream = UnixStream::connect(sock_path2).await.unwrap();

            println!("Succesfully connected to the sender. Reading file descriptor");
            let mut peer_stream = stream.recv_stream().await.unwrap();

            println!("Sending data to the peer");
            let buffer: [u8; 4] = [0, 0, 0, 42];
            peer_stream.write(&buffer).await.unwrap();
            println!("Succesfully sent data to the peer");
        });

        smol::future::zip(j1, j2).await;
    });

    let _ = std::fs::remove_dir(sock_path);
}
