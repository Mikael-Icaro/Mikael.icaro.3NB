// Importa a função que compara números (usada pra saber se é maior ou menor)
use std::cmp::Ordering;

// Estrutura para um Nó da árvore
struct Node {
    valor: i32,                      // valor que o nó guarda
    esquerda: Option<Box<Node>>,    // filho da esquerda (valores menores)
    direita: Option<Box<Node>>,     // filho da direita (valores maiores)
}

impl Node {
    // Função para criar um novo nó
    fn novo(valor: i32) -> Self {
        Node {
            valor,
            esquerda: None,
            direita: None,
        }
    }

    // Função pra inserir um novo valor na árvore
    fn inserir(&mut self, novo_valor: i32) {
        if novo_valor < self.valor {
            // se for menor, vai pra esquerda
            match self.esquerda {
                Some(ref mut filho) => filho.inserir(novo_valor), // se já tem filho, chama recursivamente
                None => {
                    self.esquerda = Some(Box::new(Node::novo(novo_valor))); // senão, cria novo nó
                }
            }
        } else if novo_valor > self.valor {
            // se for maior, vai pra direita
            match self.direita {
                Some(ref mut filho) => filho.inserir(novo_valor),
                None => {
                    self.direita = Some(Box::new(Node::novo(novo_valor)));
                }
            }
        } else {
            // se for igual, não adiciona (não vamos repetir)
            println!("Valor {} já está na árvore", novo_valor);
        }
    }

    // Mostra os valores da árvore em ordem (menor pro maior)
    fn em_ordem(&self) {
        if let Some(ref filho_esq) = self.esquerda {
            filho_esq.em_ordem();
        }

        println!("{}", self.valor);

        if let Some(ref filho_dir) = self.direita {
            filho_dir.em_ordem();
        }
    }

    // Procura um valor dentro da árvore
    fn buscar(&self, alvo: i32) -> bool {
        if alvo == self.valor {
            return true;
        } else if alvo < self.valor {
            match self.esquerda {
                Some(ref filho) => filho.buscar(alvo),
                None => false,
            }
        } else {
            match self.direita {
                Some(ref filho) => filho.buscar(alvo),
                None => false,
            }
        }
    }
}

// Estrutura principal da árvore (tem só a raiz)
struct ArvoreBinaria {
    raiz: Option<Box<Node>>,
}

impl ArvoreBinaria {
    fn nova() -> Self {
        ArvoreBinaria { raiz: None }
    }

    fn inserir(&mut self, valor: i32) {
        match self.raiz {
            Some(ref mut no) => no.inserir(valor),
            None => {
                self.raiz = Some(Box::new(Node::novo(valor)));
            }
        }
    }

    fn em_ordem(&self) {
        match self.raiz {
            Some(ref no) => no.em_ordem(),
            None => println!("A árvore está vazia."),
        }
    }

    fn buscar(&self, valor: i32) -> bool {
        match self.raiz {
            Some(ref no) => no.buscar(valor),
            None => false,
        }
    }
}

// Aqui começa o programa
fn main() {
    let mut arvore = ArvoreBinaria::nova(); // cria a árvore vazia

    // Inserindo alguns números
    arvore.inserir(10);
    arvore.inserir(5);
    arvore.inserir(15);
    arvore.inserir(3);
    arvore.inserir(7);

    println!("Valores em ordem:");
    arvore.em_ordem(); // mostra os números em ordem

    // Vamos buscar alguns números
    println!("Tem o número 7? {}", arvore.buscar(7));
    println!("Tem o número 12? {}", arvore.buscar(12));
}
