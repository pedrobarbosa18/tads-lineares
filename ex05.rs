// Exercício 02 — Contador de ocorrências de letras
//
// Complexidade:
//   - Iteração pelo Vec<char>: O(n), onde n é o número de caracteres
//     Visitamos cada char exatamente uma vez.
//   - Inserção/atualização no HashMap: O(1) amortizado por operação
//   - Total: O(n) — linear no tamanho da frase

use std::collections::HashMap;

fn contar_letras(vec: &Vec<char>) -> HashMap<char, usize> {
    let mut contagem: HashMap<char, usize> = HashMap::new();

    for &letra in vec {
        // Ignora espaços — só conta letras
        if letra != ' ' {
            // entry() evita dupla busca: se existe, incrementa; se não, insere 0 e incrementa
            *contagem.entry(letra).or_insert(0) += 1;
        }
    }

    contagem
}

fn main() {
    let frase = "estruturas de dados";
    let vec: Vec<char> = frase.chars().collect();

    println!("Frase: \"{}\"", frase);

    let contagem = contar_letras(&vec);

    // Ordena alfabeticamente para exibição organizada
    let mut pares: Vec<(char, usize)> = contagem.into_iter().collect();
    pares.sort_by_key(|&(c, _)| c);

    println!("Ocorrências:");
    for (letra, qtd) in pares {
        println!("  '{}' => {}", letra, qtd);
    }
}
