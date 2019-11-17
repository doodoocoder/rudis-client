use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::net::TcpStream;
//use rudis_client::*;

///用于测试连接的tcp模型
fn main() {
    let stream = TcpStream::connect("127.0.0.1:6379").unwrap();
    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);
    let byStr = b"*3\r\n$3\r\nSET\r\n$5\r\nmykey\r\n$7\r\nmyvalue\r\n";
    writer.write_all(byStr).unwrap();
    writer.flush().unwrap();
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    println!("{}", line);
    


    //连接模块

    //操作模块

    //异常模块

    //暴露的安全方法
}