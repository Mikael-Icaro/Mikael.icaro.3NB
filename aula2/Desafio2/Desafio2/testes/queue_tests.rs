use desafioAula2::queue::Queue;

#[test]
fn test_enqueue_dequeue() {
    // Cria uma nova fila
    let mut queue = Queue::new();

    // Adiciona elementos à fila
    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(3);

    // Verifica se os elementos são removidos na ordem correta (FIFO - First In, First Out)
    assert_eq!(queue.dequeue(), Some(1));
    assert_eq!(queue.dequeue(), Some(2));
    assert_eq!(queue.dequeue(), Some(3));

    // Verifica se a fila está vazia após remover todos os elementos
    assert_eq!(queue.dequeue(), None);
}

#[test]
fn test_peek() {
    // Cria uma nova fila
    let mut queue = Queue::new();

    // Adiciona um elemento à fila
    queue.enqueue(42);

    // Verifica se o método peek retorna uma referência para o primeiro elemento sem removê-lo
    assert_eq!(queue.peek(), Some(&42));
}

#[test]
fn test_len() {
    // Cria uma nova fila
    let mut queue = Queue::new();

    // Verifica se a fila começa vazia
    assert_eq!(queue.len(), 0);

    // Adiciona elementos à fila
    queue.enqueue(10);
    queue.enqueue(20);

    // Verifica se o tamanho da fila foi atualizado corretamente
    assert_eq!(queue.len(), 2);
}

#[test]
fn test_is_empty() {
    // Cria uma nova fila
    let mut queue = Queue::new();

    // Verifica se a fila começa vazia
    assert!(queue.is_empty());

    // Adiciona um elemento à fila
    queue.enqueue(5);

    // Verifica se a fila não está mais vazia
    assert!(!queue.is_empty());
}