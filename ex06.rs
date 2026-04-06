// Exercício 03 — Remoção condicional de números pares (sem .retain())
//
// Complexidade:
//   Abordagem 1 — Novo vetor (filtro): O(n)
//     Percorremos o vetor uma vez e copiamos só os ímpares. Simples e linear.
//
//   Abordagem 2 — Remoção in-place com índice: O(n²) no pior caso
//     Cada .remove(i) desloca todos os elementos à direita — caro!
//     Para n=10000 elementos, pode fazer ~50 milhões de operações.
//     É como tirar uma carta do meio do baralho e reempacotar.

fn remover_pares_novo_vec(vec: &Vec<i32>) -> Vec<i32> {
    // O(n): cria novo vetor só com ímpares
    let mut resultado = Vec::new();
    for &x in vec {
        if x % 2 != 0 {
            resultado.push(x);
        }
    }
    resultado
}

fn remover_pares_inplace(vec: &mut Vec<i32>) {
    // O(n²): remove elementos in-place — educativo para mostrar o custo
    let mut i = 0;
    while i < vec.len() {
        if vec[i] % 2 == 0 {
            vec.remove(i); // desloca tudo à direita: O(n) por chamada
        } else {
            i += 1;
        }
    }
}

fn main() {
    let original = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("Original: {:?}", original);

    // Abordagem 1 — preferida por ser O(n)
    let filtrado = remover_pares_novo_vec(&original);
    println!("Filtrado (novo vec, O(n)):     {:?}", filtrado);

    // Abordagem 2 — in-place porém O(n²)
    let mut copia = original.clone();
    remover_pares_inplace(&mut copia);
    println!("Filtrado (in-place, O(n²)):    {:?}", copia);

    println!("\n⚠ Análise: a abordagem in-place é O(n²) porque cada remove() desloca");
    println!("  todos os elementos seguintes. Para listas grandes, prefira a Abordagem 1.");
}
