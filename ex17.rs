// Exercício 14 — Verificação de palíndromo com VecDeque
//
// Complexidade:
//   - Construção do deque: O(n)
//   - Verificação (comparação frente vs fundo): O(n/2) = O(n)
//   - Total: O(n)
//
// Estratégia: colocamos todos os chars (sem espaços, em minúsculas) no deque.
// Depois comparamos o char da frente com o do fundo simultaneamente.
// Se em algum momento diferirem, não é palíndromo.
// É como verificar um espelho: o que está na ponta esquerda deve igualar a direita.

use std::collections::VecDeque;

fn eh_palindromo(texto: &str) -> bool {
    // Filtra só letras e dígitos, tudo em minúsculo
    let mut deque: VecDeque<char> = texto
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_lowercase().next().unwrap())
        .collect();

    // Compara as extremidades e vai aproximando ao centro
    while deque.len() > 1 {
        let frente = deque.pop_front().unwrap();
        let fundo  = deque.pop_back().unwrap();
        if frente != fundo {
            return false;
        }
    }

    true
}

fn main() {
    let frases = vec![
        "A man a plan a canal Panama",
        "racecar",
        "Was it a car or a cat I saw",
        "hello",
        "Never odd or even",
        "Rust",
        "Amor a Roma",
    ];

    println!("=== Verificação de Palíndromos ===\n");
    for frase in frases {
        let resultado = if eh_palindromo(frase) { "✓ É palíndromo" } else { "✗ Não é palíndromo" };
        println!("  \"{}\"", frase);
        println!("   → {}\n", resultado);
    }
}
