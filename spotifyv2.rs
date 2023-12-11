use std::io::BufReader;
use std::io::{self, BufRead};
use std::path::Path;
use std::u32;
use std::fs::File;
mod utiles;

fn leer_lineas<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
fn loopear_rang() -> u32{
    loop{
        println!("Escoge 5 números. \nn3: Cantidad de días desde el lanzamiento.\nn4: Cantidad de veces que estuvo en el top 10.\nn6: Cantidad de veces que estuvo en el top 1.\nn7: Número de reproducciones en su peak.\nn8: cantidad de reproducciones totales.\nn9: ACCEDE A EJERCICIO 6 ");
        let numero = utiles::texto_numero("columna".to_string());
        if numero == 3 || numero == 4 || numero == 6 || numero == 7 || numero == 8 || numero == 9{
            print_columna(numero);
            return numero;
        }
    }
}

fn print_columna(numero:u32) -> (){
    match numero{
        3 => println!("El artista y cancion desde el lanzamiento"),
        4 => println!("El artista y cancion mayor cantidad de veces en el top 10"),
        6 => println!("El artista y cancion mayor cantidad de veces en el top 1"),
        7 => println!("El artista y cancion mayor numero de reproducciones en su peak"),
        8 => println!("El artista y cancion mayor cantidad de reproducciones en general"),
        _ => (),
    }
}

fn ejercicio6() -> () {

    let palabra:String = utiles::ingreso_texto("Nombre del artista a buscar".to_string());
    println!("El artista es: {}",palabra);
    let mut cantidad = 0;

    let mut contador_lineas = 0;
    if let Ok(lines) = leer_lineas("./dataSpoti.csv") {
        for line in lines {

            if let Ok(ip) = line {
                let ip_copy = ip.clone();
                let split = ip_copy.split(";");
                let mut contador_columnas:u32 = 0;
                for s in split {
                    if contador_columnas == 1 && contador_lineas >= 1{
                        let aaaaa = s.to_lowercase().to_string();
                        println!("Clasifica como artista: {}",aaaaa);
                        if palabra == aaaaa{
                            println!("Concuerda");
                            cantidad += 1;
                        }
                    }   
                    contador_columnas += 1;
                }
            }
            contador_lineas += 1;
        }
    }
    println!("El artista {} aparece {} cantidad de veces en la base de datos", palabra,cantidad); 
    if cantidad == 0 {
        println!("No se encontró en la base de datos, porfavor ingrese bien el nombre del artista\n se reiniciará el programa.\n\n");
        procesos();
    } else {
        println!("El artista {} aparece {} cantidad de veces en la base de datos", palabra,cantidad);
    }
}

fn procesos() -> () {
    let mut num_max: u32 = 0;
    let mut arreglo_final: [String;3] = [String::new(),String::new(),String::new()];

    println!("Número de columnas a buscar:");
    let columna = loopear_rang();
    if columna == 9{
        ejercicio6();
        return;
    }
    let mut contador_lineas = 0;
    if let Ok(lines) = leer_lineas("./dataSpoti.csv") {

        for line in lines {
            let mut arreglo_provisional: [String;3] = [String::new(),String::new(),String::new()];
            if let Ok(ip) = line {
                let ip_copy = ip.clone();
                let split = ip_copy.split(";");
                let mut contador_columnas:u32 = 0;

                for s in split {
                    if contador_columnas == 1 && contador_lineas >= 1{
                        arreglo_provisional[0] = s.to_string();
                    }

                    if contador_columnas == 2 && contador_lineas >= 1{
                        arreglo_provisional[1] = s.to_string();
                    }

                    if contador_columnas == columna  && contador_lineas >= 1{
                        arreglo_provisional[2] = s.to_string();
                        if let Ok(num) = s.parse::<u32>(){
                            if utiles::numero_max(num,num_max){
                                if num_max == num{
                                    println!("{:?}",arreglo_provisional);
                                }
                                num_max = num;
                                arreglo_final = arreglo_provisional.clone();
                            }
                        }
                    }

                    contador_columnas += 1;

                }
                
            }

            contador_lineas += 1;
        }
    }
    println!("{:?}", arreglo_final);
}



fn main() {
    procesos();
}
