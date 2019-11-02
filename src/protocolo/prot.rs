pub mod prot{
    pub trait Prot{
        //fn new()->Self;
        fn validar(&mut self,valor:String)->bool;
        fn procesarPeticion<'a>(&mut self)->&'a [u8];
        fn procesarEntrada<'a>(&mut self,b:u8)->&'a [u8];
    }
    pub struct Nulo ();
    impl Prot for Nulo{
        fn validar(&mut self,valor:String)->bool{
            return false;
        }
        fn procesarPeticion<'a>(&mut self)->&'a [u8]{
            return &[];
        }
        fn procesarEntrada<'a>(&mut self,b:u8)->&'a [u8]{
            return &[];
        }
    }
}