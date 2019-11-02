use chrono::prelude::*;
use std::collections::HashMap;
use sha1::{Sha1,Digest};
#[path = "prot.rs"]
mod prot;
use crate::servidor::prot::prot::Prot;
//use crate::servidor::protocolo::prot::Prot;
extern crate base64;
pub struct Websocket{
    estado:u8,
    cabeceras:HashMap<String,String>,
    instruccion:String,
    firma:String,
    siFrameAbierto: bool,
    datos:Vec<u8>,
}
impl Websocket{
    pub fn new()->Websocket{
        Websocket{estado:1,cabeceras:HashMap::new(),instruccion:String::from(""),firma:String::from(""),siFrameAbierto:true,datos:Vec::new()}
    }
    fn leerCabecera(&mut self,valor:String)->(String,String){
        println!("[{}]",valor);
        //let mut respuesta:(String,String);
        let v:Vec<&str>=valor.splitn(2,':').collect();
        //respuesta.0=v[0].to_string();
        //respuesta.1=v[1].to_string();
        let v1=v[0].to_string();
        if(v.len()<2){
            return (v1.trim().to_string(),"".to_string());
        }
        let v2=v[1].to_string();
        
        return (v1.trim().to_string(),v2.trim().to_string());
    }
    /**
     * Procesamos los frames de datos
     */
    fn procesarFrame<'a>(&mut self,b:u8)->&'a [u8]{
        return &[];
    }
}
impl Prot for Websocket{
    /*fn new()->Self{
        Websocket {estado:0,cabeceras:HashMap::new()};
    }*/
    fn validar(&mut self,valor:String)->bool{
        match(self.estado){
            1=>{
                //Leemos las cabeceras
                let cabecera=self.leerCabecera(valor);
                if(cabecera.1==""){
                    println!("Estamos en estado 2");
                    self.estado=2;
                }else{
                    self.cabeceras.insert(cabecera.0,cabecera.1);
                }
                return true;
            },
            2=>{

            },
            _=>{
               println!("WEBSOCKET {}--",valor);
            }
        }
        
        return true;
    }
    
    fn procesarPeticion<'a>(&mut self)->&'a [u8]{
        match(self.estado){
            2=>{
                //base64_encode(sha1("x3JJHMbDL1EzLkh9GBhXDw=="."258EAFA5-E914-47DA-95CA-C5AB0DC85B11",true));
                let mut clave = Sha1::new();
                let cadenaClave=String::from((self.cabeceras[&"Sec-WebSocket-Key".to_string()].clone()+"258EAFA5-E914-47DA-95CA-C5AB0DC85B11").to_owned());
                //let cadenaClave=("x3JJHMbDL1EzLkh9GBhXDw==258EAFA5-E914-47DA-95CA-C5AB0DC85B11");
                //println!("CAdena ----{}----",cadenaClave);
                clave.input(&cadenaClave.as_bytes());
                
                let claveSHA1=clave.result();
                //println!("{:#?}",claveSHA1);

                //println!("{}",);
                let parte1=String::from("HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: ");
                let parte2=String::from("\r\n\r\n");//Sec-WebSocket-Protocol: chat;\r\n\r\n
                self.estado=3;
                println!("Tengo la clave");
                self.firma=parte1+&base64::encode(claveSHA1.as_ref())+&parte2;
                //let retornar=firma.as_bytes()[..];
                let v=firma.as.vec![..];
                return v.as_bytes();//(firma.as_bytes()).clone();
            },
            
            _=>{
                return &[];
            }
        }
    }
    fn procesarEntrada<'a>(&mut self,b:u8)->&'a [u8]{
        if(self.estado==3){
            return self.procesarFrame(b);
        }else{
            if b==0{
                return &[];
            }
            if b==10{
                //println!("PRocesamos");
                if self.validar(self.instruccion.clone()){
                    self.instruccion=String::from("");
                    return self.procesarPeticion();
                };
                self.instruccion=String::from("");
            }else{
                if(b!=13){
                    self.instruccion.push(b.clone() as char);
                }
            }
            return &[];
        }
    }
}
