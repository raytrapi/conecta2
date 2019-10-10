use std::io::prelude::*;
use std::io;
use std::io::Read;
use std::net::{TcpListener, TcpStream,IpAddr, Ipv4Addr, SocketAddr};
use std::thread;


#[path = "protocolo/mod.rs"]
mod protocolo;

pub fn ejecutar(){
    let listener = TcpListener::bind("127.0.0.1:8084").unwrap();
  
    loop{
        match listener.accept() {
            Ok((_socket, addr)) => {
                thread::spawn(move || {
                    println!("new client: {:?}", addr);
                    
                    let wsRespuesta="HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: obandoxxx256\r\n";
                    let mut stream=_socket;
                    //stream.
                    stream.write_all(wsRespuesta.as_bytes());
                    let mut buf=[1;1];
                    //let mut cadena=String::new();
                    //let mut stream=_socket;
                    let cadena="";
                    let mut instruccion=String::from("");
                    loop{
                        let resultado=stream.read(&mut buf) ;
                        let mut lector:protocolo::Lector=protocolo::Lector{estado:0,tipo_protocolo:protocolo::TIPO_PROTOCOLO::SINDEFINIR};
                        match resultado{
                            Ok(resultado) => {
                                
                                if buf[0]==0{
                                    break;
                                }
                                if buf[0]==10{
                                    lector.validar(instruccion);
                                    instruccion=String::from("");
                                }else{
                                    if(buf[0]!=13){
                                        instruccion.push(buf[0] as char);
                                        
                                    }
                                }
                                
                            },
                            Err(e) => {
                                //println!("salgo");
                                break;
                            }
                        }
                        
                        
                    };
                    println!();
                    println!("El cliente se ha desconectado");
                });
            
                
                //let mut buf=[0,100];
                //
                //let resultado=_socket.read_timeout().unwrap();
                //println!("{:?}",resultado);
            },
            Err(e) => println!("couldn't get client: {:?}", e),
        }
    }
}