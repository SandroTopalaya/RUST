use rand::Rng; // Generar números aleatorios

fn main() {
    let mut rng = rand::thread_rng(); // Crear el generador de números aleatorios

    // Repite el proceso 10 veces
    for _ in 0..10 {
        let numero_aleatorio: u32 = rng.gen_range(1..=10); // Genera un número entre 1 y 10

        if numero_aleatorio == 3 || numero_aleatorio == 4 || numero_aleatorio == 5 {
            println!("Error: El número aleatorio fue {}", numero_aleatorio);
        } else {
            println!("HOLA MUNDO - El número aleatorio fue {}", numero_aleatorio);
        }
    }
}

