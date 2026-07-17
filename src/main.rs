use core::error;
use std::mem::transmute_copy;
use std::io;
use std::io::Write;

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
        fn esta_disponible(&self) -> bool;
        fn prestar (&mut self) -> bool;
        fn devolver(&mut self);
        
    }
    impl Prestable for Libros{
        fn titulo (&self)->String{
            self.titulo.clone()
        }
        fn esta_disponible(&self) -> bool {
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
    impl Revistas{
        pub fn constructor(titulo:String, edicion:u32)->Revistas{
            Revistas { titulo, edicion, disponible: true }
        }
    }
    impl Prestable for Revistas{
        fn titulo(&self) -> String {
        format!("{} - edición {}", self.titulo, self.edicion)
        }
        fn esta_disponible(&self) -> bool {
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
    fn mostrar_estado(item: &dyn Prestable) {
    let estado = if item.esta_disponible() { "disponible" } else { "prestado" };
    println!("{} está {}", item.titulo(), estado);
}
    struct Estanterias{
        nombre:String,
        coleccion:Vec<Box<dyn Prestable>>
    }
    impl Estanterias{
        pub fn constructor(nombre:String)->Estanterias{
            Estanterias { nombre, coleccion:Vec::new() }
        }

        pub fn agregar_coleccion(&self, item: Box<dyn Prestable>){
           self.coleccion.push(item);
        }
        pub fn listar(&self){
        println!("-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-");
        println!("|                            |");
        println!("|    Estantería de   {}      |", self.nombre);
        println!("|                            |");
        println!("-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-");
        println!();
        for listas in self.coleccion{
            let estado= if listas.esta_disponible() {" Disponible "} else {" No Disponible "};
                println!("1.{} => {}", self.nombre, estado);
        }
        }
    }



















let mut coleccion: Vec<Box<dyn Prestable>> = Vec::new();
let mut documento:u32;
let mut documentos:String;
loop{
    println!("-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-");
    println!("|                            |");
    println!("|         GESTIÓN DE         |");
    println!("|         BIBLIOTECA         |");
    println!("|                            |");
    println!("-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-");
    println!();
    println!("-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_");
    println!("|                            |");
    println!("|         1. Libros.         |");
    println!("|         2. Revistas        |");
    println!("|                            |");
    println!("-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_");
    println!();
    print!("Por favor, ingrese el tipo de documento con el que desea operar: ");
    documentos=String::new();
    io::stdout().flush().expect("Error en el forzamiento del buffer.");
    io::stdin().read_line(&mut documentos).expect("Error en la lectura de la linea");
    documento=match documentos.trim().parse(){
        Ok(valido)=> valido,
        Err(_)=>{
            println!();
            println!("Por favor, ingrese un número entero positivo.");
            println!();
            continue;
        }
    };
}
let mut opcion:u32;
loop{


    println!("-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-");
    println!("|                                     |");
    println!("|       1. Agregar Documentos         |");
    println!("|   2. Modificamiento de Documentos   |");
    println!("|      3. Documentos Registrados      |");
    println!("|        4. Buscar Documentos         |");
    println!("|       5. Eliminar Documentos        |");
    println!("|              6. Salir               |");
    println!("|                                     |");
    println!("-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-");
    println!();
    print!("Por favor, ingrese una opción: ");
    let mut menu:String=String::new();
    io::stdout().flush().expect("Error en el forzamiento del buffer.");
    io::stdin().read_line(&mut menu).expect("Error en la lectura de la linea");
    opcion =match menu.trim().parse(){
        Ok(valido)=> valido,
        Err(_)=>{
            println!();
            println!("Por favor, ingrese un número entero positivo.");
            println!();
            continue;
        }
    };
    println!();
    match opcion{
        1=>{

        },
        2=>{

        },
        3=>{

        },
        4=>{

        },
        5=>{

        },
        6=>{
            println!("Saliendo del programa..."); break;
        },
        _=>{
            println!("Opción inválida, por favor ingrese una opción que este en el menú\nPor favor, vuelve a intentarlo: ");
        }

    }


}

}
    



