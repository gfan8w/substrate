use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time;

fn handle_client(mut stream: TcpStream) -> Result<(), Error>{
    let mut buf = [0; 5]; //创建一个缓冲区，用来保存数据
    for _ in 0..1000 {
        let bytes_read = stream.read(&mut buf)?; //读取数据
        if bytes_read == 0 {
            return Ok(());
        }

        stream.write(&buf[..bytes_read])?;  //将读取到的内容返写到客户端去
        thread::sleep(time::Duration::from_secs(1 as u64));
    }

    Ok(()) //没有异常，返回 void
}

pub fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();

    for stream in listener.incoming() {
        let stream = stream.expect("failed!");
        //启动一个新线程来处理客户端的数据流
        let handle = thread::spawn(move || {        //move 使stream可以穿越移动
            handle_client(stream)
                .unwrap_or_else(|error| eprintln!("{:?}", error)); //如果遇到错误，会打印错误，错误由 stream的read，write 发出
        });

        thread_vec.push(handle);//把每个线程都保存起来
    }

    //等待，不要让服务端运行结束
    for handle in thread_vec {
        handle.join().unwrap();
    }

    Ok(())
}

//运行方法：
// main();



