use std::io::prelude::*;
use std::io;
use std::io::Read;
use std::net::{TcpListener, TcpStream,IpAddr, Ipv4Addr, SocketAddr};
pub fn ejecutar(){
    let mut stream = TcpStream::connect("127.0.0.1:8084").unwrap();//.expect("Couldn't connect to the server...");
    println!("Connected to the server!");
    println!("{:?}",stream);
    
    
    /*stream.flush();
    */
    
    loop{
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                if input=="EXIT\r\n"{
                    print!("FINALIZO");
                    break;
                }
                stream.write_all(input.as_bytes());
            }
            Err(error) =>{
                 break;
            }
        }
    }
    let mut b=[0;1];
    b[0]=0;
    stream.write(&mut b);
    /*let mut b:[u8;1];
    b[0]=0;
    stream.write(&mut b);*/
    println!("Termino");

}