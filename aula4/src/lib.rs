// Remove o 'pub' pois a função não precisa ser pública neste exemplo
// Adiciona documentação explicando os requisitos de segurança
/// Multiplica todos os elementos de um array de inteiros
/// 
/// # Safety
/// - O ponteiro deve ser válido e apontar para pelo menos `len` elementos
/// - O array deve conter elementos inicializados do tipo i32
unsafe fn multiply_array(ptr: *const i32, len: usize) -> i32 {
    let mut product = 1;
    for i in 0..len {  // Corrige o índice inicial para 0
        // Usa add() para aritmética de ponteiros (mais idiomático que offset)
        product *= *ptr.add(i); 
    }
    product
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply_array() {
        let arr = [2, 3, 4];
        // Testa caso básico
        unsafe {
            assert_eq!(multiply_array(arr.as_ptr(), arr.len()), 24);
        }
        
        // Adiciona teste para array vazio
        let empty_arr: [i32; 0] = [];
        unsafe {
            assert_eq!(multiply_array(empty_arr.as_ptr(), empty_arr.len()), 1);
        }
        
        // Adiciona teste com zero
        let arr_with_zero = [2, 0, 4];
        unsafe {
            assert_eq!(multiply_array(arr_with_zero.as_ptr(), arr_with_zero.len()), 0);
        }
    }
}

fn main() {
    println!("Hello, world!");
}
