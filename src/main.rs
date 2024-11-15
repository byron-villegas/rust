use std::io::{stdin,stdout, Write};

mod cajero;

fn main() {
    let mut opcion = -1;

    while opcion != 0 {
        println!("======= MENU =======");
        
        println!("1): Cajero");
        println!("0): Salir");
        
        println!("");

        print!("Por favor seleccione una opcion: ");
        let _ = stdout().flush();

        let mut entrada = String::new();
        
        stdin().read_line(&mut entrada).expect("Error: Por favor ingrese una opcion valida.");

        match entrada.trim().parse::<i32>() {
            Ok(value) => {
                opcion = value;
            }
            Err(_) => {
                eprintln!("Error: Por favor ingrese un numero.");
            }
        }

        println!("");

        match opcion {
            0 => println!("Muchas gracias por utilizar el menu."),
            1 => cajero::run(),
            _ => println!("Opcion no encontrada.")
        }

        println!("");
    }
}