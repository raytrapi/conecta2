use chrono::prelude::*;
mod websocket;
pub enum TIPO_PROTOCOLO{
    SINDEFINIR,
    P2P,
    WEBSOCKET
}
pub struct Lector{
    pub estado:u32,
    pub tipo_protocolo:TIPO_PROTOCOLO,
    
}
impl Lector{
    pub fn validar(&mut self, valor:String)->bool{
        match self.tipo_protocolo{
            TIPO_PROTOCOLO::SINDEFINIR=>{
                println!("ENTRO");
                if(valor=="GET / HTTP/1.1"){
                    self.tipo_protocolo=TIPO_PROTOCOLO::WEBSOCKET;
                    //println!("{}",self.tipo_protocolo);
                    return websocket::validar(valor);
                }else{
                    self.tipo_protocolo=TIPO_PROTOCOLO::P2P;
                }
            },
            TIPO_PROTOCOLO::WEBSOCKET=>{
                return websocket::validar(valor);
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
}
