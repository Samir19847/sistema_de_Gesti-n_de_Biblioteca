
use std::io;
use std::io::Write;
use std::str::CharIndices;
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
        pub fn devolver(&mut self)->bool{
            if !self.disponible{
                self.disponible=true;
                true
            }
            else{
                false
            }
        }
    
    }
    trait Prestable {
        fn titulo(&self) -> String;
        fn esta_disponible(&self) -> bool;
        fn prestar (&mut self) -> bool;
        fn devolver(&mut self) ->bool;
        
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
        fn devolver(&mut self) ->bool{
            if !self.disponible{
                self.disponible=true;
                true    
            }
            else {
                false
            }
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
        fn devolver(&mut self) ->bool {
            if !self.disponible{
                self.disponible = true;
                true
            }
            else {
                false
            }
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
        pub fn constructor(nombre:String, coleccion:Vec<Box<dyn Prestable>>)->Estanterias{
            Estanterias { nombre, coleccion}
        }

        pub fn agregar_coleccion(&mut self, item: Box<dyn Prestable>){
           self.coleccion.push(item);
        }
        pub fn listar(&self){
        println!("-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-");
        println!("|                                    |");
        println!("|    Colección de la {}      |", self.nombre);
        println!("|                                    |");
        println!("-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-");
        println!();
        for listas in &self.coleccion{
            let estado= if listas.esta_disponible() {" Disponible "} else {" No Disponible "};
                println!("1.{} => {}", listas.titulo(), estado);
        }
        }
    }
















let nombree:String=format!("Biblioteca");
let mut coleccion: Vec<Box<dyn Prestable>> = Vec::new();
let mut estanteria:Estanterias=Estanterias::constructor(nombree, coleccion);
let mut coleccion_libros:Vec<Libros>=Vec::new();
let mut coleccion_revistas:Vec<Revistas>=Vec::new();
let mut documento:u32;
let mut documentos:String;
let mut obj_libros:String;
let mut titulazo:String;
let mut autorazo:String;
let mut edicionazo:String;
let mut contador:u32=1;
let mut opcion:u32;


loop{
    documento=loop{
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
        println!("|         2. Revistas.       |");
        println!("| 9. Salir del a aplicación. |");
        println!("|                            |");
        println!("-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_");
        println!();
        print!("Por favor, ingrese el tipo de documento con el que desea operar: ");
        documentos=String::new();
        io::stdout().flush().expect("Error en el forzamiento del buffer.");
        io::stdin().read_line(&mut documentos).expect("Error en la lectura de la linea");
        match documentos.trim().parse(){
            Ok(valido)=> break valido,
            Err(_)=>{
                println!();
                println!("Por favor, ingrese un número entero positivo.");
                println!();
                continue;
            }
        }
    
    };
    if documento==9{
        println!("Saliendo del programa...");
        break;
    }
    loop{
        println!("-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-");
        println!("|                                     |");
        println!("|       1. Agregar Documentos         |");
        println!("|        2. Buscar Documentos         |");
        println!("|      3. Documentos Registrados      |");
        println!("|   4. Modificamiento de Documentos   |");
        println!("|       5. Eliminar Documentos        |");
        println!("|         6. Ver Estanterías          |");
        println!("|    7. Salir del tipo de documento   |");
        println!("|       8. Prestar Documento          |");
        println!("|       9. Devolver Documento         |");
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
                if documento==1{
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

                    let obj_libros=Libros::constructor(titulazo, autorazo);
                    coleccion_libros.push(obj_libros.clone());
                    estanteria.agregar_coleccion(Box::new(obj_libros));


                    print!("Libro guardado correctamente.");
                    println!();

                }
                else if documento==2{
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

                    let obj_revistas=Revistas::constructor(titulazo, edicionazo);
                    
                    coleccion_revistas.push(obj_revistas.clone());
                    estanteria.agregar_coleccion(Box::new(obj_revistas));
                    print!("Revista guardada correctamente.");
                    println!();
                }
            },
            2=>{
                if documento==1{
                    if coleccion_libros.len()>0{
                        print!("Por favor, ingrese el título del libro que desea buscar: ");
                        let mut buscado:String=String::new();
                        io::stdout().flush().expect("Error en el forzamiento del bufer. ");
                        io::stdin().read_line(&mut buscado).expect("Error en la lectura de la línea");
                        let titulo_buscado=buscado.trim();
                        println!();
                        if coleccion_libros.iter().any(|libro| libro.titulo==titulo_buscado)
                        {
                            println!("¡El libro {} sí existe en la colección! - ", titulo_buscado);
                            match coleccion_libros.iter().position(|posicion| posicion.titulo==titulo_buscado) {
                                Some(indice) => {
                                    match coleccion_libros.get(indice) {
                                        Some(libro) => println!("El libro en el índice {} es {} - {}", indice, libro.titulo, libro.autor),
                                        None => println!("No hay ningún libro en la posición {}.", indice),
                                    }
                                    }
                                
                                None =>{
                                    println!();
                                }
                            }
                            match estanteria.coleccion.iter().position(|posicion| posicion.titulo()==titulo_buscado) {
                                Some(indice) => {
                                    println!("Su posición entre la lista general de la biblioteca es de {} unidades", indice)
                                }
                                None=>{
                                    println!();
                                }
                            }
                            }
                        else{
                            println!("El libro {} no se encuentra en la colección.", titulo_buscado)
                            }
                        }
                        else{
                        println!("No se ha guardado ningúun libro aún.\nPor favor, agregue un libro por lo menos.");}
                            
                }
                else if documento==2{
                    if coleccion_revistas.len()>0{
                    print!("Por favor, ingrese el título de la revista que desea buscar: ");
                    let mut buscado1:String=String::new();
                    io::stdout().flush().expect("Error en el forzamiento del bufer. ");
                    io::stdin().read_line(&mut buscado1).expect("Error en la lectura de la línea");
                    let titulo_buscado1=buscado1.trim();
                    println!();
                    if coleccion_revistas.iter().any(|libro| libro.titulo==titulo_buscado1)
                    {
                    match coleccion_revistas.iter().position(|posicion| posicion.titulo == titulo_buscado1) {
                        Some(indice) => {
                            match coleccion_revistas.get(indice) { 
                            Some(revista) => {
                            println!("La revista en el índice {} es {}", indice, revista.titulo());
                            }
                            None => {} 
                        }
                        }
                        None => {
                        println!(); 
                     }
                    }
                    }
                    
                    else {
                        println!("El libro {} no se encuentra en la colección.", titulo_buscado1)
                        }
                        }
                    else{
                        println!("No se ha guardado ninguna revista aún.\nPor favor, agregue una revista por lo menos.");
                    }
                }
            },
            3=>{
                if documento==1{
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
                        let mut contador=1;
                        for item in &coleccion_libros{
                            let estado= if item.esta_disponible() {" Disponible "} else {" No Disponible "};
                            println!("{}. Título: {}, Autor: {}, Estado: {}.", contador, item.titulo, item.autor, estado);
                            println!();
                            contador+=1;
                        }
                            
                    }
                    else{
                        println!("No se ha guardado ningún libro aún.\nPor favor, agregue un libro por lo menos.");
                    }
                }
                else if documento==2{
                    if coleccion_revistas.len()>0{
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
                        let mut contador=1;
                        for item in &coleccion_revistas{
                            let estado= if item.esta_disponible() {" Disponible "} else {" No Disponible "};
                            println!("{}. Título: {}, Autor: {}, Estado: {}.", contador, item.titulo, item.edicion, estado);
                            println!();
                            contador+=1;
                        }
                    }
                    else{
                        println!("No se ha guardado ninguna revista aún.\nPor favor, agregue una revista por lo menos.");
                    }
                }
            },
            4=>{
                if documento==1{
                    print!("Por favor, ingrese el título del libro que desea buscar: ");
                    let mut modificado1:String=String::new();
                    io::stdout().flush().expect("Error en el forzamiento del bufer.");
                    io::stdin().read_line(&mut modificado1).expect("Error en la lectura de la línea");
                    let titulo_modificado1=modificado1.trim();
                    println!();
                    if coleccion_libros.iter().any(|libros| libros.titulo==modificado1) {
                            println!("¡El libro {} fue entonctrado!", modificado1);
                            if let Some(indice)=coleccion_libros.iter().position(|libros| libros.titulo==modificado1) {
                                if let Some(libro)=coleccion_libros.get_mut(indice){
                                    print!("Por favor, ingrese el nuevo título del libro: {}", modificado1);
                                    let mut nuevo1:String=String::new();
                                    io::stdout().flush().expect("Error en el forzamiento del bufer.");
                                    io::stdin().read_line(&mut nuevo1).expect("Error en la lectura de la línea");
                                    let titulo_nuebo1=nuevo1.trim().to_string();
                                    libro.titulo=titulo_nuebo1;
                                    println!();
                                    print!("Por favor, ingrese el autor del libro: {}", nuevo1);
                                    let mut nuevo2:String=String::new();
                                    io::stdout().flush().expect("Error en el forzamiento del bufer.");
                                    io::stdin().read_line(&mut nuevo2).expect("Error en la lectura de la línea");
                                    let autor_nuebo1=nuevo2.trim().to_string();
                                    libro.autor=autor_nuebo1;
                                }
                            }
                            println!();
                            println!("Cambios realizados.");
                                
                    }
                    else{
                        println!("El libro no fue encontrado...");
                    }
                }
                else if documento==2{
                    print!("Por favor, ingrese el título de la revista que desea buscar: ");
                    let mut modificado2:String=String::new();
                    io::stdout().flush().expect("Error en el forzamiento del bufer.");
                    io::stdin().read_line(&mut modificado2).expect("Error en la lectura de la línea");
                    let titulo_modificado2=modificado2.trim();
                    println!();
                    if coleccion_revistas.iter().any(|libros| libros.titulo==modificado2) {
                            println!("!La revista {} fue entonctrado!", modificado2);
                            if let Some(indice)=coleccion_revistas.iter().position(|libros| libros.titulo==modificado2) {
                                if let Some(revistas)=coleccion_revistas.get_mut(indice){
                                    print!("Por favor, ingrese el nuevo título del libro: {}", modificado2);
                                    let mut nuevo2:String=String::new();
                                    io::stdout().flush().expect("Error en el forzamiento del bufer.");
                                    io::stdin().read_line(&mut nuevo2).expect("Error en la lectura de la línea");
                                    let titulo_nuebo2=nuevo2.trim().to_string();
                                    revistas.titulo=titulo_nuebo2;
                                    println!();
                                    print!("Por favor, ingrese la edición de la revista {}: ", nuevo2);
                                    let mut nuevo3:String=String::new();
                                    io::stdout().flush().expect("Error en el forzamiento del bufer.");
                                    io::stdin().read_line(&mut nuevo3).expect("Error en la lectura de la línea");
                                    let edicion_nuebo2=nuevo3.trim().to_string();
                                    revistas.edicion=edicion_nuebo2;
                                }
                            }
                            println!();
                            println!("Cambios realizados.");
                                
                    }
                    else{
                        println!("La revista no fue encontrada...");
                    }
                }
            },
            5=>{
                if documento==1{
                if coleccion_libros.len()>0{
                    if coleccion_libros.len()>0{
                    print!("Por favor, ingrese el título del libro que desea buscar: ");
                    let mut eliminado1:String=String::new();
                    io::stdout().flush().expect("Error en el forzamiento del bufer. ");
                    io::stdin().read_line(&mut eliminado1).expect("Error en la lectura de la línea");
                    let titulo_eliminado1=eliminado1.trim();
                    println!();
                    if let Some(indice)=coleccion_libros.iter().position(|libros|libros.titulo==titulo_eliminado1){
                        coleccion_libros.remove(indice); 
                        println!("¡El libro ha sido eliminado con éxito!");
                    }
                    else{
                        println!("Libro no encontrado.");
                    }
                    if let Some(indice)=estanteria.coleccion.iter().position(|rasgo| rasgo.titulo()==titulo_eliminado1){
                            estanteria.coleccion.remove(indice);
                        }
                }
                else{
                    println!("No se ha guardado ningún libro aún.\nPor favor, agregue un libro por lo menos.");
                }
                }
                else{
                    println!("No se ha guardado ningún libro aún.\nPor favor, agregue un libro por lo menos.");
                }
                }
                else if documento==2{
                    if coleccion_revistas.len()>0{
                        if coleccion_revistas.len()>0{
                        print!("Por favor, ingrese el título de la revista: ");
                        let mut eliminado2:String=String::new();
                        io::stdout().flush().expect("Error en el forzamiento del bufer.");
                        io::stdin().read_line(&mut eliminado2).expect("Error en la lectura de la línea.");
                        let titulo_eliminado2=eliminado2.trim();
                        println!();
                        if let Some(indice)=coleccion_revistas.iter().position(|revistas|revistas.titulo==titulo_eliminado2){
                            coleccion_revistas.remove(indice);
                            println!("¡La revista fue eliminada con éxito!");
                        }
                        else {
                            println!("Revista no encontrada...");
                        }
                        if let Some(indice)=estanteria.coleccion.iter().position(|rasgo| rasgo.titulo()==titulo_eliminado2){
                            estanteria.coleccion.remove(indice);
                        }
                    }
                    else{
                        println!("No se ha guardado ninguna revista aún.\nPor favor, agregue una revista por lo menos.");
                    }
                    }
                    else{
                        println!("No se ha guardado ninguna revista aún.\nPor favor, agregue una revista por lo menos.");
                    }
                }
            },
            6=>{
                if estanteria.coleccion.len()>0{
                    estanteria.listar();
                }
                else{
                    println!("No hay ningún tipo de documento en la colección.\nPor favor, ingresa por lo menos un tipo de documento.");
                }
            },
            7=>{
                println!("Saliendo del tipo de documento...");
                println!(); break;
            }
            8 => {
                if documento == 1 {
                    if coleccion_libros.len()>0{
                        print!("Ingrese el título del libro a prestar: ");
                        let mut buscado: String = String::new();
                        io::stdout().flush().expect("Error en el forzamiento del bufer.");
                        io::stdin().read_line(&mut buscado).expect("Error en la lectura de la línea");
                        let titulo_buscado = buscado.trim();
                        println!();
                        if let Some(indice) = coleccion_libros.iter().position(|l| l.titulo == titulo_buscado) {
                        if let Some(libro) = coleccion_libros.get_mut(indice) {
                            if libro.prestar() {
                                println!("Libro prestado con éxito.");
                            } else {
                                println!("El libro ya estaba prestado.");
                            }
                        }
                        } else {
                        println!("Libro no encontrado.");
                        }
                    }
                    else {
                        println!("No hay ningún libro guardado aún.\nPor favor, ingrese por lo menos un libro.");
                    }
                }
            if documento==2{
                if coleccion_revistas.len()>0{
                    print!("Ingrese el título de la revista a prestar: ");
                    let mut buscado1:String=String::new();
                    io::stdout().flush().expect("Error en el forzamiento del bufer");
                    io::stdin().read_line(&mut buscado1).expect("Error en la lectura de la línea");
                    let titulo_buscado1=buscado1.trim();
                    println!();
                    if let Some(indice)=coleccion_revistas.iter().position(|r| r.titulo==titulo_buscado1){
                        if let Some(revista)=coleccion_revistas.get_mut(indice){
                            if revista.prestar() {
                                println!("Revista prestada con éxito.");
                        }
                            else {
                                println!("La revista ya estaba prestada.")
                            }
                        }
                    }
                    else{
                    println!("Revista no encontrada");
                    }  
                }
                else {
                    println!("No hay ninguna revista registrada aún.\nPor favor, ingrese por lo menos alguna revista.");
                }
            }
        },
            9=>{
                if documento==1{
                    if coleccion_libros.len()>0{
                        print!("Ingrese el título del libro a devolver: ");
                        let mut buscado:String=String::new();
                        io::stdout().flush().expect("Error en el forzamiento del búfer");
                        io::stdin().read_line(&mut buscado).expect("Error en la lectura de la línea.");
                        let titulo_buscado=buscado.trim();
                        println!();
                        if let Some(indice)=coleccion_libros.iter().position(|l|l.titulo==titulo_buscado){
                            if let Some(libro)=coleccion_libros.get_mut(indice){
                                if libro.devolver() {
                                    println!("El libro ha sido devolvido con exito.");
                                }
                                else{
                                    println!("El libro ya estaba devolvido...");
                                }
                            }
                        }
                        else {
                            println!("El libro no se ha encontrado en ninguna lista.");
                        }

                    }
                    else{
                        println!("No hay ningún libro guardado.\nPor favor, ingrese al menos un libro.");
                    }                    
                }
                else if documento==2 {
                    if coleccion_revistas.len()>0{
                        print!("Ingrese el título del libro a devolver: ");
                        let mut buscado1:String=String::new();
                        io::stdout().flush().expect("Error en el forzamiento del búfer");
                        io::stdin().read_line(&mut buscado1).expect("Error en la lectura de la línea.");
                        let titulo_buscado1=buscado1.trim();
                        println!();
                        if let Some(indice)=coleccion_revistas.iter().position(|l|l.titulo==titulo_buscado1){
                            if let Some(revista)=coleccion_revistas.get_mut(indice){
                                if revista.devolver() {
                                    println!("¡La revista ha sido devolvida con exito.");
                                }
                                else{
                                    println!("La revista ya estaba devolvida...");
                                }
                            }
                        }
                        else {
                            println!("La revista no se ha encontrado en ninguna lista.");
                        }

                    }
                    else {
                        println!("No se ha almacenado ninguna revista aún.\nPor favor, ingrese una alguna revista.");
                    }
                }
           
            },
            _=>{
                println!("Opción inválida, por favor ingrese una opción que este en el menú\nPor favor, vuelve a intentarlo: ");
            }

        }
    }    
}

}