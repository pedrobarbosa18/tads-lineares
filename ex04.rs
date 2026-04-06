// Exercício 01 — Inversão com Vec
//
// Complexidade:
//   - Inverter usando push/pop: O(n)
//     Passamos por cada elemento uma vez para empilhar e uma vez para desempilhar.
//     É como desvirar uma pilha de pratos: pega um por um e coloca em ordem inversa.

fn inverter(vec: &Vec<i32>) -> Vec<i32> {
    let mut pilha = vec.clone();
    let mut resultado = Vec::new();

    // Retiramos do fim do vetor (pop) e empurramos no novo (push)
    // O último elemento sai primeiro — comportamento LIFO natural do Vec
    while let Some(valor) = pilha.pop() {
        resultado.push(valor);
    }

    resultado
}

fn main() {
    let original = vec![1, 2, 3, 4, 5];
    println!("Original : {:?}", original);

    let invertido = inverter(&original);
    println!("Invertido: {:?}", invertido);
}
