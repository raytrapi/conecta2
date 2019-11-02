use chrono::prelude::*;
use std::collections::HashMap;
////extern crate crypto;
//use sha1::{Sha1,Digest};
//extern crate base64;
mod websocket;
use crate::servidor::protocolo::websocket::Websocket;
mod prot;
use crate::servidor::prot::prot::Prot;
pub enum TIPO_PROTOCOLO{
    SINDEFINIR,
    P2P,
    WEBSOCKET
}

pub struct Protocolo{
    pub estado:u32,
    pub tipo_protocolo:TIPO_PROTOCOLO,
    pub protocolo:Box<Prot>,
}
impl Protocolo{
    pub fn validar(&mut self, valor:String)->bool{
        match self.tipo_protocolo{
            TIPO_PROTOCOLO::SINDEFINIR=>{
                
                if(valor=="GET / HTTP/1.1"){
                    self.protocolo=Box::new(Websocket::new());
                    self.estado=1;
                    self.tipo_protocolo=TIPO_PROTOCOLO::WEBSOCKET;
                    
                    return true;
                }else{
                    self.tipo_protocolo=TIPO_PROTOCOLO::P2P;
                }
            },
            TIPO_PROTOCOLO::WEBSOCKET=>{
                return self.protocolo.validar(valor);
            },
            TIPO_PROTOCOLO::P2P=>{}
        }
        if valor=="DATE"{
            println!("{:?}",Utc::now());
            println!("{:?}",Local::now());
        }else{
            println!("{:?}",valor);
        }
        return valor!="H";
    }
    pub fn procesarPeticion<'a>(&mut self)->&'a [u8]{
         match self.tipo_protocolo{
            TIPO_PROTOCOLO::SINDEFINIR=>{
            },
            TIPO_PROTOCOLO::WEBSOCKET=>{
                return self.protocolo.procesarPeticion();
            },
            TIPO_PROTOCOLO::P2P=>{}
        }
        return &[];
    }
    pub fn procesarEntrada<'a>(&mut self,b:u8)->&'a [u8]{
        return self.protocolo.procesarEntrada(b);
    }
}
