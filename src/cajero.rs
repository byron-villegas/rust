use std::io::{stdin,stdout, Write};

pub fn run() {
    let mut opcion = -1;

    let mut saldo = 0;

    while opcion != 0 {
        println!("======= CAJERO =======");
        
        println!("1) Consultar Saldo");
        println!("2) Ingresar Deposito");
        println!("3) Girar Dinero");
        println!("0) Salir");

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

        match opcion {
            0 => println!("Muchas gracias por usar el cajero."),
            1 => consultar_saldo(saldo),
            2 => {
                saldo = ingresar_deposito(saldo);
            },
            3 => {
                saldo = girar(saldo);
            }
            _ => println!("Opcion no encontrada.")
        }

        println!("");
    }
}

fn consultar_saldo(saldo: i32) {
    println!("Saldo actual {0}", saldo);
}

fn ingresar_deposito(saldo: i32) -> i32 {
    print!("Ingrese monto a depositar: ");
    let _ = stdout().flush();

    let mut monto: i32 = 0;

    let mut entrada = String::new();
    
    stdin().read_line(&mut entrada).expect("Error: Por favor ingrese un monto valido.");

    match entrada.trim().parse::<i32>() {
        Ok(value) => {
            monto = value;
        }
        Err(_) => {
            eprintln!("Error: Por favor ingrese un monto valido.");
        }
    }

    if monto < 1 {
        eprintln!("Error: El monto no puede ser menor a 1");
        return saldo;
    }

    println!("Se ha depositado {0} de forma exitosa.", monto);

    let resultado = saldo + monto;

    println!("El nuevo saldo es {0}", resultado);
    
    return saldo + monto;
}

fn girar(saldo: i32) -> i32 {
    print!("Ingrese monto a girar: ");
    let _ = stdout().flush();

    let mut monto: i32 = 0;

    let mut entrada = String::new();
    
    stdin().read_line(&mut entrada).expect("Error: Por favor ingrese un monto a girar valido.");

    match entrada.trim().parse::<i32>() {
        Ok(value) => {
            monto = value;
        }
        Err(_) => {
            eprintln!("Error: Por favor ingrese un monto a girar valido.");
        }
    }

    if monto < 1 {
        eprintln!("Error: El monto a girar no puede ser menor a 1");
        return 0;
    }

    let resultado = saldo - monto;

    if resultado <= -1 {
        eprintln!("Error: el monto a girar {0} no puede superar el saldo actual {1}", monto, saldo);
        return saldo;
    }


    println!("El nuevo saldo es {0}", resultado);

    return resultado;
}