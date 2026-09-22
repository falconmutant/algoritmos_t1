use std::io;

mod matrix;
mod bubble;
mod roman_to_number;
mod mcd;
mod prime_number;
mod palindrome;


fn main() {
    loop {
        println!();
        println!("==============================");
        println!("       MENÚ DE ALGORITMOS");
        println!("==============================");
        println!("1. Voltear matriz");
        println!("2. Transponer matriz");
        println!("3. Ordenamiento burbuja");
        println!("4. Romano a entero");
        println!("5. Máximo común divisor");
        println!("6. Número primo");
        println!("7. Palíndromo");
        println!("8. Elemento mayor de una matriz");
        println!("0. Salir");
        println!("==============================");

        let option = read_number("Selecciona una opción:");

        match option {
            1 => {
                let mut values = read_matrix();

                println!("Matriz original:");
                print_matrix(&values);

                matrix::flip(&mut values);

                println!("Matriz volteada:");
                print_matrix(&values);
            }

            2 => {
                let values = read_matrix();

                let result = matrix::transpose(&values);

                println!("Matriz original:");
                print_matrix(&values);

                println!("Matriz transpuesta:");
                print_matrix(&result);
            }

            3 => {
                let mut values = read_vector();

                println!("Vector original: {:?}", values);

                bubble::sort(&mut values);

                println!("Vector ordenado: {:?}", values);
            }

            4 => {
                let roman_number = read_text("Ingresa un número romano:");

                let result = roman_to_number::parse(&roman_number);

                println!("{} = {}", roman_number, result);
            }

            5 => {
                let num1 = read_number("Ingresa el primer número:");
                let num2 = read_number("Ingresa el segundo número:");

                let result = mcd::mcd(num1, num2);

                println!("MCD de {} y {} = {}", num1, num2, result);
            }

            6 => {
                let number = read_number("Ingresa un número:");

                let result = prime_number::is_prime(number);

                println!("¿{} es primo? {}", number, result);
            }

            7 => {
                let text = read_text("Ingresa un texto:");

                let result = palindrome::is_palindrome(&text);

                println!("¿{} es palíndromo? {}", text, result);
            }

            8 => {
                let values = read_matrix();

                let result = matrix::major(&values);

                println!("El elemento mayor es: {}", result);
            }

            0 => {
                println!("Programa finalizado.");
                break;
            }

            _ => {
                println!("Opción no válida. Selecciona una opción del 0 al 8.");
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

fn read_text(message: &str) -> String {
    println!("{}", message);

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Error al leer");

    input.trim().to_string()
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

fn read_matrix() -> Vec<Vec<i32>> {
    let rows = read_positive_number("Número de filas:");
    let columns = read_positive_number("Número de columnas:");

    let mut matrix = Vec::new();

    for i in 0..rows {
        loop {
            println!(
                "Ingresa la fila {} con {} números separados por espacios:",
                i + 1,
                columns
            );

            let mut input = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("Error al leer");

            let mut row = Vec::new();
            let mut valid = true;

            for value in input.split_whitespace() {
                match value.parse::<i32>() {
                    Ok(number) => {
                        row.push(number);
                    }

                    Err(_) => {
                        valid = false;
                        break;
                    }
                }
            }

            if !valid {
                println!("La fila contiene un valor que no es un número.");
                continue;
            }

            if row.len() != columns {
                println!(
                    "Debes ingresar exactamente {} números. Ingresaste {}.",
                    columns,
                    row.len()
                );

                continue;
            }

            matrix.push(row);
            break;
        }
    }

    matrix
}

fn read_positive_number(message: &str) -> usize {
    loop {
        let number = read_number(message);

        if number > 0 {
            return number as usize;
        }

        println!("El número debe ser mayor que 0.");
    }
}

fn print_matrix(matrix: &Vec<Vec<i32>>) {
    for row in matrix {
        println!("{:?}", row);
    }
}