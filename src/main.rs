use std::mem::transmute_copy;

fn main() {
    struct Libros{
        autor:String,
        titulo:String,
        disponible:bool,
    }

    impl Libros{
        pub fn constructor(titulo:String, autor:String)->Libros{
            Libros { autor, titulo, disponible: true }
        }
        pub fn esta_disponible(&self) -> bool {
        self.disponible
    }
        pub fn prestar(&mut self)->bool{
            if self.disponible{
                self.disponible=false;
                true
            }
            else {
                false
            }
        }
        pub fn devolver(&mut self){
            self.disponible=true;
        }

    
    }
    trait Prestable {
        fn titulo(&self) -> String;
        fn estadisponible(&self) -> bool;
        fn prestar (&mut self) -> bool;
        fn devolver(&mut self);
        
    }
    impl Prestable for Libros{
        fn titulo (&self)->String{
            self.titulo.clone()
        }
        fn estadisponible(&self) -> bool {
            self.disponible
        }
        fn prestar (&mut self) -> bool {
            if self.disponible {
            self.disponible = false;
            true
        } else {
            false
        }
        }
        fn devolver(&mut self) {
            self.disponible=true;
        }
        
    }
    struct Revistas{
        titulo:String,
        edicion:u32,
        disponible:bool,
}
    impl Prestable for Revistas{
        fn titulo(&self) -> String {
        format!("{} - edición {}", self.titulo, self.edicion)
        }
        fn estadisponible(&self) -> bool {
        self.disponible
        }
        fn prestar(&mut self) -> bool {
            if self.disponible {
                self.disponible = false;
                true
            } 
            else {
            false
            }
        }
        fn devolver(&mut self) {
            self.disponible = true;
        }
    }
}
    



