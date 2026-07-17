
use std::io;
use std::io::Write;

fn main() {
    #[derive(Clone)]
    struct Libros {
        titulo:String,
        autor:String,
        disponible:bool,
    }

    impl Libros{
        pub fn constructor(titulo:String, autor:String)->Libros{
            Libros { titulo, autor, disponible: true }
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
     #[derive(Clone)]
    struct Revistas{
        titulo:String,
        edicion:String,
        disponible:bool,
}
    impl Revistas{
        pub fn constructor(titulo:String, edicion:String)->Revistas{
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

        pub fn agregar_coleccion(&mut self, item: Box<dyn Prestable>){
           self.coleccion.push(item);
        }
        pub fn listar(&self){
        println!("-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-");
        println!("|                            |");
        println!("|    Estantería de   {}      |", self.nombre);
        println!("|                            |");
        println!("-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-");
        println!();
        for listas in &self.coleccion{
            let estado= if listas.esta_disponible() {" Disponible "} else {" No Disponible "};
                println!("1.{} => {}", self.nombre, estado);
        }
        }
    }



















let mut coleccion: Vec<Box<dyn Prestable>> = Vec::new();
let mut coleccion_libros:Vec<Libros>=Vec::new();
let mut coleccion_revistas:Vec<Revistas>=Vec::new();
let documento:u32;
let mut documentos:String;
let mut obj_libros:String;
let mut titulazo:String;
let mut autorazo:String;
let mut edicionazo:String;
let mut contador:u32=1;




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
    match documentos.trim().parse(){
        Ok(valido)=> {
            documento=valido;
            break;
        }
        Err(_)=>{
            println!();
            println!("Por favor, ingrese un número entero positivo.");
            println!();
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
    println!("|         6. Ver Estanterías          |");
    println!("|              7. Salir               |");
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
            if opcion==1{
                print!("Por favor, ingrese el título del libro: {contador}: ");
                let mut titulo:String=String::new();
                io::stdout().flush().expect("Error en el forzamiento del bufer. ");
                io::stdin().read_line(&mut titulo).expect("Error en la lectura de la línea");
                titulazo=titulo.trim().to_string();
                println!();

                print!("Por favor, ingrese el autor del libro: {contador}: ");
                let mut autor:String=String::new();
                io::stdout().flush().expect("Error en el forzamiento del bufer. ");
                io::stdin().read_line(&mut autor).expect("Error en la lectura de la línea");
                autorazo=autor.trim().to_string();
                println!();
                contador+=1;

                let mut obj_libros=Libros::constructor(titulazo, autorazo);
                coleccion_libros.push(obj_libros.clone());
                coleccion.push(Box::new(obj_libros));
                print!("Libro guardado correctamente.");
                println!();

            }
            else if opcion==2{
                print!("Por favor, ingrese el título de la revista: {contador}: ");
                let mut titulo:String=String::new();
                io::stdout().flush().expect("Error en el forzamiento del bufer. ");
                io::stdin().read_line(&mut titulo).expect("Error en la lectura de la línea");
                titulazo=titulo.trim().to_string();
                println!();

                print!("Por favor, ingrese la edición de la revista: {contador}: ");
                let mut edicion:String=String::new();
                io::stdout().flush().expect("Error en el forzamiento del bufer. ");
                io::stdin().read_line(&mut edicion).expect("Error en la lectura de la línea");
                edicionazo=edicion.trim().to_string();
                println!();
                contador+=1;

                let mut obj_revistas=Revistas::constructor(titulo, edicion);
                
                coleccion_revistas.push(obj_revistas.clone());
                coleccion.push(Box::new(obj_revistas));
                print!("Revista guardada correctamente.");
                println!();
            }
        },
        2=>{
            if opcion==1{
                if coleccion_libros.len()>0{

                }
                else{
                    println!("No se ha guardado ningpun libro aún.\nPor favor, agregue un libro por lo menos.");
                }
            }
            else if opcion==2{
                if coleccion_revistas.len()>0{

                }
                else{
                    println!("No se ha guardado ningpuna revista aún.\nPor favor, agregue una revista por lo menos.");
                }
            }
        },
        3=>{
            if opcion==1{
                if coleccion_libros.len()>0{
                        println!("-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-");
                        println!("|                            |");
                        println!("|         GESTIÓN DE         |");
                        println!("|         BIBLIOTECA         |");
                        println!("|                            |");
                        println!("-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-");
                        println!();
                        println!("-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_");
                        println!("|                            |");
                        println!("|         Documentos         |");
                        println!("|         Registrados        |");
                        println!("|                            |");
                        println!("-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_");
                        println!();
                        for item in &coleccion_libros{
                            println!("Título: {}, Autor: {}, Estado: {}.", item.titulo, item.autor, item.disponible);
                            println!();
                        }
                }
                else{
                    println!("No se ha guardado ningpun libro aún.\nPor favor, agregue un libro por lo menos.");
                }
            }
            else if opcion==2{
                if coleccion_revistas.len()>0{

                }
                else{
                    println!("No se ha guardado ningpuna revista aún.\nPor favor, agregue una revista por lo menos.");
                }
            }
        },
        4=>{
            if opcion==1{
            if coleccion_libros.len()>0{

            }
            else{
                println!("No se ha guardado ningpun libro aún.\nPor favor, agregue un libro por lo menos.");
            }
            }
            else if opcion==2{
                if coleccion_revistas.len()>0{

                }
                else{
                    println!("No se ha guardado ningpuna revista aún.\nPor favor, agregue una revista por lo menos.");
                }
            }
        },
        5=>{
            if opcion==1{
            if coleccion_libros.len()>0{

            }
            else{
                println!("No se ha guardado ningpun libro aún.\nPor favor, agregue un libro por lo menos.");
            }
            }
            else if opcion==2{
                if coleccion_revistas.len()>0{

                }
                else{
                    println!("No se ha guardado ningpuna revista aún.\nPor favor, agregue una revista por lo menos.");
                }
            }
        },
        6=>{
            if coleccion.len()>0{

            }
            else{
                println!("No hay ningún libro en la colección.\nPor favor, ingresa por lo menos un tipo de documento.");
            }
        },
        7=>{
             println!("Saliendo del programa..."); break;
        }
        _=>{
            println!("Opción inválida, por favor ingrese una opción que este en el menú\nPor favor, vuelve a intentarlo: ");
        }

    }


}

}
    



