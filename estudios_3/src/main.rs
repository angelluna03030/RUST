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
// struct Punto<T> {
//     x: T,
//     y: T,
// }
// las estructuras pueden tener metodos con el mismo normbre,
// lo esencial es llamar a la structura 

// IMPORTANTE: ya que se van a utulizar muchas veces, se debe utlizar el trait,
// asi RUST entendera que un metodo que se ta utilizar en muchos casos pero diferentes struct


// pub  trait  Resumen {
//     fn resumir(&self) -> String;
//     // aqui le estoy diciendo que si no hay una implementacion que tenga funcion que 
//     // se llame resumir_contenido, entonces que use la implementacion por defecto
//     //asi nos aseguramos que ten tenga esas funcinon por defecto 
    
//     fn resumir_contenido(&self) -> String {
//         String::from("Contenido por defecto")
//     }
// }
// pub struct Noticia {
//     pub titular: String,
//     pub autor: String,
//     pub contenido: String,
//     pub localizacion: String,

// }

// impl Resumen for Noticia {
//     fn resumir(&self) -> String {
//         format!("{} por {}", self.titular, self.autor)
//     }
    
// }
// pub struct Tweet {
//     pub usuario: String,
//     pub contenido: String,
//     pub retweets: bool,
//     pub respuesta: bool,
// }

// impl Resumen for Tweet {
//     fn resumir(&self) -> String {
//         format!("{}: {}", self.usuario, self.contenido)
//     }
    
// }
fn main() {

    // let tweet = Tweet {
    //     usuario: String::from("usuario123"),
    //     contenido: String::from("Este es un tweet"),
    //     retweets: true,
    //     respuesta: false,
    // };
    // println!("Resumen del tweet: {}", tweet.resumir());
    // let noticia = Noticia {
    //     titular: String::from("Titular de la noticia"),
    //     autor: String::from("Autor de la noticia"),
    //     contenido: String::from("Contenido de la noticia"),
    //     localizacion: String::from("Localización de la noticia"),
    // };

    // println!("Resumen de la noticia: {}", noticia.resumir());
    // //self hace referencia al datoa que le estamos pasando en si mismo

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
    // impl<T> Punto<T> {
    //     fn X(&self) -> &T {
    //        &self.x
    //     }

    // }
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }
    // enum Result<T,E>{
    //     Ok(T),
    //     Err(E),
    // }
    // let lista = vec![1, 2, 3, 4, 5];
    // let resultado = mayor(&lista);
    // println!("El mayor es: {}", resultado);
    // let lista_de_caracteres = vec!['z', 'b', 'c', 'd', 'x'];
    // let resultado = mayor(&lista_de_caracteres);
    // println!("El mayor es: {}", resultado);
}
