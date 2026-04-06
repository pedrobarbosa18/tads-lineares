// Exercício 07 — Editor de texto com Desfazer/Refazer
//
// Complexidade:
//   - digitar(texto): O(1) — push nas pilhas
//   - desfazer():     O(1) — pop do undo, push no redo
//   - refazer():      O(1) — pop do redo, push no undo
//
// Cada estado do editor é uma String. Ao digitar, o estado atual
// vai para a pilha de undo e o novo estado vira o atual.
// É exatamente como o Ctrl+Z/Ctrl+Y de qualquer editor.

struct Editor {
    conteudo: String,
    pilha_undo: Vec<String>, // histórico de estados anteriores
    pilha_redo: Vec<String>, // estados "desfeitos" que podem ser refeitos
}

impl Editor {
    fn new() -> Self {
        Editor {
            conteudo: String::new(),
            pilha_undo: Vec::new(),
            pilha_redo: Vec::new(),
        }
    }

    fn digitar(&mut self, texto: &str) {
        // Salva estado atual antes de mudar
        self.pilha_undo.push(self.conteudo.clone());
        // Nova ação elimina o futuro (redo limpo)
        self.pilha_redo.clear();
        self.conteudo.push_str(texto);
        self.mostrar("Digitou");
    }

    fn desfazer(&mut self) {
        if let Some(estado_anterior) = self.pilha_undo.pop() {
            // Estado atual vai para redo antes de ser substituído
            self.pilha_redo.push(self.conteudo.clone());
            self.conteudo = estado_anterior;
            self.mostrar("Desfez");
        } else {
            println!("  [nada para desfazer]");
        }
    }

    fn refazer(&mut self) {
        if let Some(estado_futuro) = self.pilha_redo.pop() {
            self.pilha_undo.push(self.conteudo.clone());
            self.conteudo = estado_futuro;
            self.mostrar("Refez");
        } else {
            println!("  [nada para refazer]");
        }
    }

    fn mostrar(&self, acao: &str) {
        println!("  [{}] Conteúdo: \"{}\"", acao, self.conteudo);
    }
}

fn main() {
    let mut editor = Editor::new();

    editor.digitar("Olá");
    editor.digitar(", mundo");
    editor.digitar("!");
    editor.desfazer();
    editor.desfazer();
    editor.refazer();
    editor.digitar(" Rust"); // ação nova: limpa redo
    editor.refazer();        // não há mais o que refazer
    editor.desfazer();
    editor.desfazer();
    editor.desfazer();
    editor.desfazer();       // além do início
}
