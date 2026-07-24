# Sistema de Gestión de Biblioteca

Aplicación de consola en Rust para administrar el catálogo de una biblioteca, con soporte para dos tipos de material: **libros** y **revistas**. Permite registrar, buscar, modificar, eliminar y controlar el préstamo/devolución de cada documento.

## Funcionalidades

- Registrar libros (título, autor) y revistas (título, edición)
- Buscar un documento por título y ver su posición en el catálogo
- Listar todos los documentos registrados, agrupados en una estantería general
- Modificar los datos de un documento existente
- Eliminar documentos del catálogo
- Prestar y devolver documentos, con control de disponibilidad

## Conceptos de Rust aplicados

Este proyecto fue una excusa para practicar polimorfismo en Rust sin herencia clásica:

- **Trait `Prestable`**: define el comportamiento común entre `Libros` y `Revistas` (`titulo`, `esta_disponible`, `prestar`, `devolver`)
- **Trait objects (`Box<dyn Prestable>`)**: la estructura `Estanterias` guarda ambos tipos de documento en una misma colección, sin importar cuál sea
- **Ownership y borrowing**: manejo de referencias mutables (`&mut self`) para actualizar el estado de disponibilidad
- **Pattern matching**: `match` para el menú de opciones y para manejar resultados de `Option`/`Result`

## Cómo ejecutarlo

Necesitás tener [Rust y Cargo](https://www.rust-lang.org/tools/install) instalados.

```bash
git clone https://github.com/Samir19847/sistema_de_Gesti_n_de_Biblioteca.git
cd sistema_de_Gesti_n_de_Biblioteca
cargo run
```

## Estado del proyecto

Proyecto funcional de consola, hecho como práctica de structs, traits y manejo de colecciones en Rust. Pendiente de mejoras futuras: persistencia en archivo/base de datos y separación del código en módulos.
