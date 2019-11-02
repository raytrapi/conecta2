use std::io::prelude::*;
use std::io;
use std::io::Read;
use std::net::{TcpListener, TcpStream,IpAddr, Ipv4Addr, SocketAddr};
use std::thread;
use std::collections::HashMap;


#[path = "protocolo/mod.rs"]
mod protocolo;
#[path = "protocolo/prot.rs"]
pub mod prot;
use crate::servidor::prot::prot::Nulo;


pub fn ejecutar(){
    let listener = TcpListener::bind("127.0.0.1:8084").unwrap();
  
    loop{
        match listener.accept() {
            Ok((_socket, addr)) => {
                thread::spawn(move || {
                    println!("new client: {:?}", addr);
                    
                    //let wsRespuesta="HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: obandoxxx256\r\n";
                    let mut stream=_socket;
                    //stream.
                    //stream.write_all(wsRespuesta.as_bytes());
                    let mut buf=[1;1];
                    //let mut cadena=String::new();
                    //let mut stream=_socket;
                    let cadena="";
                    let mut instruccion=String::from("");
                    let mut servidor:protocolo::Protocolo=protocolo::Protocolo{estado:0,tipo_protocolo:protocolo::TIPO_PROTOCOLO::SINDEFINIR,protocolo: Box::new(Nulo())};
                    loop{
                        let resultado=stream.read(&mut buf) ;
                        match resultado{
                            Ok(resultado) => {
                                match servidor.tipo_protocolo{
                                    protocolo::TIPO_PROTOCOLO::WEBSOCKET=>{
                                        let respuesta=servidor.procesarEntrada(buf[0]);
                                        if(respuesta.len()>0){
                                            stream.write_all(respuesta);
                                        }
                                    },
                                    _=>{
                                        if buf[0]==0{
                                            break;
                                        }
                                        if buf[0]==10{
                                            println!("PRocesamos");
                                            if servidor.validar(instruccion){
                                                let respuesta=servidor.procesarPeticion();
                                                if (respuesta.len()>0){
                                                    println!("{:#?}",respuesta);
                                                    stream.write_all(respuesta);
                                                }
                                            };
                                            
                                            instruccion=String::from("");
                                        }else{
                                            if(buf[0]!=13){
                                                instruccion.push(buf[0] as char);
                                                
                                            }
                                        }

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