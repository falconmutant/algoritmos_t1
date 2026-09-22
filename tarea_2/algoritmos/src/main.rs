use std::io;

mod binary;
mod insertion;

fn main() {
    loop {
        println!();
        println!("==============================");
        println!("       MENÚ DE ALGORITMOS");
        println!("==============================");
        println!("1. Ordenamiento por Inserción Ascendente");
        println!("2. Ordenamiento por Inserción Descendente");
        println!("3. Suma de binarios");
        println!("0. Salir");
        println!("==============================");

        let option = read_number("Selecciona una opción:");

        match option {
            1 => {
                let mut values = read_vector();

                println!("Arreglo original: {:?}", values);
                insertion::sort_ascending(&mut values);

                println!("Arreglo ordenado: {:?}", values);
            }

            2 => {
                let mut values = read_vector();

                println!("Arreglo original: {:?}", values);
                insertion::sort_descending(&mut values);

                println!("Arreglo ordenado: {:?}", values);
            }

            3 => {
                loop {
                    let a = read_binary("Ingresa el primer número binario:");
                    let b = read_binary("Ingresa el segundo número binario:");

                    if a.len() != b.len() {
                        println!("Los números binarios deben tener la misma cantidad de bits.");
                        println!("Intenta nuevamente.");
                        continue;
                    }

                    let result = binary::sum(&a, &b);

                    print!("A:         ");
                    print_binary(&a);
                    print!("B:         ");
                    print_binary(&b);
                    print!("Resultado: ");
                    print_binary(&result);

                    break;
                }
            }

            0 => {
                println!("Programa finalizado.");
                break;
            }

            _ => {
                println!("Opción no válida. Selecciona una opción del 0 al 3.");
            }
        }
    }
}


fn read_number(message: &str) -> i32 {
    loop {
        println!("{}", message);

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Error al leer");

        match input.trim().parse() {
            Ok(number) => return number,

            Err(_) => {
                println!("Entrada inválida. Debes ingresar un número.");
            }
        }
    }
}

fn read_vector() -> Vec<i32> {
    loop {
        println!("Ingresa los números separados por espacios:");
        println!("Ejemplo: 5 3 8 1 2");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Error al leer");

        let mut values = Vec::new();
        let mut valid = true;

        for value in input.split_whitespace() {
            match value.parse::<i32>() {
                Ok(number) => {
                    values.push(number);
                }

                Err(_) => {
                    valid = false;
                    break;
                }
            }
        }

        if valid && !values.is_empty() {
            return values;
        }

        println!("Entrada inválida. Intenta nuevamente.");
    }
}

fn read_binary(message: &str) -> Vec<i32> {
    loop {
        println!("{}", message);

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Error al leer");

        let input = input.trim();

        let mut binary = Vec::new();
        let mut valid = true;

        for bit in input.chars() {
            if bit == '0' {
                binary.push(0);
            } else if bit == '1' {
                binary.push(1);
            } else {
                valid = false;
                break;
            }
        }

        if valid && !binary.is_empty() {
            return binary;
        }

        println!("Número binario inválido. Solo puedes utilizar 0 y 1.");
    }
}

fn print_binary(binary: &Vec<i32>) {
    for bit in binary {
        print!("{}", bit);
    }

    println!();
}