use std::io;
use std::fs::File;
use std::path::Path;
use std::fs::OpenOptions;


pub fn num(campo:String) -> u32 {
    loop {
        println!("Ingrese un número para el/la {}: ",campo);
        let mut numero = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut numero).unwrap();
        let numero: u32 = match numero.trim().parse(){
            Ok(numero) => numero,
            Err(_) => {
                println!("Error, no es un número");
                continue;
            },
        };
        return numero;
    }
}

// ESTO SIRVE PARA AÑADIR NOTAAAAAS
pub fn num_float() -> f32 {
    loop {
        println!("Ingrese un número: ");
        let mut numero = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut numero).unwrap();
        let numero: f32 = match numero.trim().parse(){
            Ok(numero) => numero,
            Err(_) => {
                println!("Error, no es un número");
                continue;
            },
        };
        if numero <= 7.0{
            println!("");
            return numero;
        } else {
            println!("ingrese un numero entre el 1 y el 7");
        }
    }
}


pub fn input_texto(campo: String) -> String {

    println!("Ingrese {}", campo);
    let mut texto = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut texto).unwrap();
    return texto

}

pub fn si_o_no() -> bool {

    loop{
        println!("digite 1 para un SI, 0 para un NO\n");
        let desicion = num("desicion".to_string());
        if desicion == 1 {
            return true;
        }
        if desicion == 0{
            return false;
        } 
    }
}


pub fn numero_max(numero1:u32,numero2:u32) -> bool{

    if numero1 >= numero2{
        return true;    
    } else {
        return false;
    }
}

pub fn crear_archivo(p: &Path) {
    let _file = File::create(p).expect("El archivo no pudo crearse");
    println!("El archivo fue creado");
}
pub fn abrir_y_editar(p: &Path) -> File{
    let mut binding = OpenOptions::new();
    let binding = binding.append(true);
    let file = match binding.open(p){
        Err(_why) => panic!("No se puede abrir el archivo"),
        Ok(file) => file,
    };
    return file
}
pub fn abrir(p: &Path) -> (){
    crear_archivo(p);
    let _file = match File::open(&p){
        Err(_why) => panic!("El archivo no se puede abrir..."),
        Ok(file) => file,
    };
}


