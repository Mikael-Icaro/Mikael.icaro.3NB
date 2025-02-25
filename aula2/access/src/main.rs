fn main() {
    let numeros = vec![5, 10, 15, 20];

    println!("Primeiro elemento: {}", numeros[0]);

    // Acessando elementos com `get()` para evitar erros
    match numeros.get(10) {
        Some(valor) => println!("Elemento na posição 10: {}", valor),
        None => println!("Posição inválida!"),
    }
}