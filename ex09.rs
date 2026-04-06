// Exercício 06 — Histórico de navegação com Voltar/Avançar
//
// Complexidade:
//   - visitar(url): O(1) — push na pilha de back, limpa forward
//   - voltar():     O(1) — pop de back, push em forward
//   - avancar():    O(1) — pop de forward, push em back
//
// Intuição: duas pilhas de pratos. "Voltar" move o prato do topo
// da pilha de trás para a de frente, e vice-versa.

struct Navegador {
    atual: Option<String>,
    historico_back: Vec<String>,    // pilha de páginas anteriores
    historico_forward: Vec<String>, // pilha de páginas futuras (após voltar)
}

impl Navegador {
    fn new() -> Self {
        Navegador {
            atual: None,
            historico_back: Vec::new(),
            historico_forward: Vec::new(),
        }
    }

    fn visitar(&mut self, url: &str) {
        // Ao visitar nova página, a atual vai para o histórico back
        // e o forward é limpo (não há mais "futuro")
        if let Some(pagina_atual) = self.atual.take() {
            self.historico_back.push(pagina_atual);
        }
        self.historico_forward.clear();
        self.atual = Some(url.to_string());
        println!("  Visitando: {}", url);
    }

    fn voltar(&mut self) {
        if let Some(anterior) = self.historico_back.pop() {
            // A página atual vai para o forward antes de sair
            if let Some(pagina_atual) = self.atual.take() {
                self.historico_forward.push(pagina_atual);
            }
            self.atual = Some(anterior);
            println!("  Voltando para: {}", self.atual.as_ref().unwrap());
        } else {
            println!("  [sem histórico para voltar]");
        }
    }

    fn avancar(&mut self) {
        if let Some(proxima) = self.historico_forward.pop() {
            if let Some(pagina_atual) = self.atual.take() {
                self.historico_back.push(pagina_atual);
            }
            self.atual = Some(proxima);
            println!("  Avançando para: {}", self.atual.as_ref().unwrap());
        } else {
            println!("  [sem páginas para avançar]");
        }
    }

    fn pagina_atual(&self) {
        match &self.atual {
            Some(p) => println!("  Página atual: {}", p),
            None => println!("  Nenhuma página aberta"),
        }
    }
}

fn main() {
    let mut nav = Navegador::new();

    nav.visitar("google.com");
    nav.visitar("wikipedia.org");
    nav.visitar("rust-lang.org");
    nav.pagina_atual();

    nav.voltar();
    nav.pagina_atual();

    nav.voltar();
    nav.pagina_atual();

    nav.avancar();
    nav.pagina_atual();

    nav.visitar("github.com"); // limpa o forward
    nav.avancar();             // não há para onde avançar
}
