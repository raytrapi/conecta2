use std::io::prelude::*;
use std::io;
use std::io::Read;
use std::fs::File;
use std::env;
use std::thread;
use std::net::{TcpListener, TcpStream,IpAddr, Ipv4Addr, SocketAddr};
mod cliente;
mod servidor;
fn main() {
    
    let args: Vec<String> = env::args().collect();
    if args.len()>1{
        if args[1]=="c"{
            
            cliente::ejecutar();
         } else if args[1]=="s" {
            servidor::ejecutar();
        }
    }
    println!("Se ha terminado");
    
}

