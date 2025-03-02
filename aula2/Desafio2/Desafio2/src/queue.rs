use std::ptr; // Importa o módulo ptr para manipular ponteiros crus

// Estrutura que representa um nó da fila
pub struct Node<T> {
    value: T, // Armazena o valor do nó
    next: Option<Box<Node<T>>>,
}

// Estrutura que representa a fila
pub struct Queue<T> {
    head: Option<Box<Node<T>>>, // Ponteiro para o primeiro nó da fila
    tail: *mut Node<T>, // Ponteiro para o último nó da fila
    len: usize, // Número de elementos na fila
}

impl<T> Queue<T> {
    // Cria uma nova fila vazia
    pub fn new() -> Self {
        Queue {
            head: None,
            tail: ptr::null_mut(),
            len: 0,
        }
    }

    // Adiciona um elemento à fila
    pub fn enqueue(&mut self, elem: T) {
        let mut new_node = Box::new(Node {
            value: elem,
            next: None,
        });

        let raw_node = &mut *new_node as *mut _;

        if self.tail.is_null() {
            self.head = Some(new_node);
        } else {
            unsafe {
                (*self.tail).next = Some(new_node);
            }
        }

        self.tail = raw_node;
        self.len += 1;
    }

    // Remove e retorna o primeiro elemento da fila
    pub fn dequeue(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            if self.head.is_none() {
                self.tail = ptr::null_mut();
            }
            self.len -= 1;
            node.value
        })
    }

    // Retorna uma referência ao primeiro elemento sem removê-lo
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    // Retorna o tamanho da fila
    pub fn len(&self) -> usize {
        self.len
    }

    // Verifica se a fila está vazia
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

// Libera a memória da fila ao ser descartada
impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        while self.dequeue().is_some() {}
    }
}