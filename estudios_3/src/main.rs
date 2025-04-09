// fn mayor_i32(lista: &[i32]) -> i32 {
//     let mut mayor = lista[0];
//     for &item in lista {
//         if item > mayor {
//             mayor = item;
//         }
//     }
//     mayor
// }
// fn mayor_char(list: &[char]) -> char {
//     let mut mayor = list[0];
//     for &item in list {
//         if item > mayor {
//             mayor = item;
//         }
//     }
//     mayor
// }

// fn mayor<T>(lista: &[T]) -> T
// {
//     let mut mayor = lista[0];
//     for &item in lista {
//         if item > mayor {
//             mayor = item;
//         }
//     }
//     mayor
// }
struct Punto<T> {
    x: T,
    y: T,   
}
fn main() {

    
    //TTPOS DE DATOS GENERICOS
    // let entero = Punto{x: 1, y: 2};
    // let decimal = Punto{x: 1.0, y: 2.0};
    // let caracter = Punto{x: 'a', y: 'b'};
    // println!("entero: ({}, {})", entero.x, entero.y);
    // let fallara = Punto{x: 1, y: 'a'}; // Error de tipo
    
    // let lista = vec![1, 2, 3, 4, 5];
    // let resultado = mayor_i32(&lista);
    // println!("El mayor es: {}", resultado);
    // let lista_de_caracteres = vec!['z', 'b', 'c', 'd', 'x'];
    // let resultado = mayor_char(&lista_de_caracteres);
    // println!("El mayor es: {}", resultado);
    impl<T> Punto<T> {
        fn X(&self) -> &T {
           &self.x
        }
        
    }
    enum Option<T> {
        Some(T),
        None,
    }
    enum Result<T,E>{
        Ok(T),
        Err(E),
    }
    // let lista = vec![1, 2, 3, 4, 5];
    // let resultado = mayor(&lista);
    // println!("El mayor es: {}", resultado);
    // let lista_de_caracteres = vec!['z', 'b', 'c', 'd', 'x'];
    // let resultado = mayor(&lista_de_caracteres);
    // println!("El mayor es: {}", resultado);
    
}
