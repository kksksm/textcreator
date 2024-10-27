
use std::{fs::File, io::Write};

fn main(){

    println!("Bienvenido! comienza a escribir!");
    let mut contenido = String::new();
    std::io::stdin().read_line(&mut contenido).unwrap();
    println!("cual es el nombre de este archivo?");
    let mut nombre_del_archivo = String::new();
    std::io::stdin().read_line(&mut nombre_del_archivo).unwrap();
    println!("cual es la extension de este archivo?");
    let mut extension = String::new();
    std::io::stdin().read_line(&mut extension).unwrap();
    let mut crear_archivo = File::create(nombre_del_archivo.trim_end().to_owned()+&extension.trim_end()).unwrap();
    crear_archivo.write_all(contenido.as_bytes()).unwrap();
    println!("el archivo ha sido guardado");
}
