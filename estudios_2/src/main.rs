// use std::collections::HashMap;

//el Option sirve para tener un valor opcional y el el valor como tal
/*
pub enum Option<T> {
    None,// no puede conteenr nada
    Some(Valor), // puede contener un valor,
}
*/
// fn incrementar_uno(x: Option<i32>) -> Option<i32> {
//     //match sirve para el control de flujo ose para que verificar que todo esta bien definido
//     // o para que cunado este verificado algo se pueda ejecutar el proceso
//     match x {
//         Some(valor) => Some(valor + 1),
//         None => None,
//     }
// }

// #[derive(Debug)]
// enum  Mes {
//     Enero,
//     Febrero,
//     Marzo,
//     Abril,
//     Mayo,
//     Junio,
//     Julio,
//     Agosto,
//     Septiembre,
//     Octubre,
//     Noviembre,
//     Diciembre
// }
// enum Tiempo {
//     Segundo,
//     Minuto,
//     Hora,
//     Dia(Mes),

// }

use std::fs::File;
fn main() {
    //panic!("eror");
    let vector = vec![1, 2, 3, 4, 5];
    let f = File::open("archivo.txt");

    let aux = vector[9];
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
    let f = match f {
        Ok(fichero) => fichero,
        Err(error) => {
            println!("Error al abrir el archivo: {:?}", error);
            return;
        }
    };
//   let nombre_campo = String::from("Nombre");
// let aquipos = vec![
//     String::from("Juan"), String::from("Pedro"), String::from("Maria")];
//     let puntuaciones_iniciales  = vec![10, 20, 30];
//     let mut puntuaciones : HashMap<String, i32> = 
//     aquipos.into_iter().zip(puntuaciones_iniciales.into_iter()).collect();

    // let mut puncuaciones = HashMap::new();
    // puncuaciones.insert(String::from("Warriors"), 121);
    // puncuaciones.insert(String::from("Lakers"), 123);
    // for (clave , valor ) in &puncuaciones {
    //     println!("El equipo {} tiene una puntuacion de {}", clave, valor);
    // }
    // println!("El equipo Warriors tiene una puntuacion de {}", puncuaciones["Warriors"]);
    //    let tiempo = Tiempo::Dia((Mes::Octubre));
    //    let mut contador = 0;

    //    if let  Tiempo::Dia(mes) = tiempo {
    //     println!("El mes es: {:?}", mes);
    //     }else {
    //         contador += 1;
    //     }

    //     //imprime el mes de enero

    //     println!("El contador es: {}", contador);

    // let maxima_config: Option<u8> = None;
    //     let  maxima_config = Some(7u8);
    //     //el condicional if let permite verificar el valor al instate si
    //     //desechar a los otros.

    //    if let Some(maxima) = maxima_config {
    //         println!("La maxima configuracion es: {}", maxima);
    //     } else {
    //         println!("No hay maxima configuracion");
    //     }

    // let valor = Some(5);
    // let resultado = incrementar_uno(valor);
    // println!("{:?}", resultado); // Imprime Some(6)

    // let valor_nulo: Option<i32> = Some(5);
    // let resultado_nulo = incrementar_uno(valor_nulo);
    // println!("{:?}", resultado_nulo); // Imprime None

    // VECTORES DE DATOS
    // solo sirven para almacenar datos del mismo tipo
    //     let mut x: Vec<i32> = Vec::new();
    //     x.push(1);
    //     x.push(232);
    //     x.push(323);
    //     println!("{:?}", x[2]);

    //     x.pop(); // elimina el ultimo elemento del vector
    //     println!("{:?}", x.len()); // imprime la longitud del vector
    //     let v = vec![1, 2, 3, 4, 5, 3];
    //     let tercer: &i32 = &v[2]; // se obtiene el tercer elemento del vector
    //     println!("{:?}", tercer); // imprime el tercer elemento del vector
    //     match v.get(2) {
    //         Some(valor) => println!("El tercer elemento es: {}", valor),
    //         None => println!("No hay tercer elemento"),
    //     }
    //     for i in &v {
    //         println!("El elemento es: {}", i);
    //     }

    //     //para almacenar datos de diferentes tipos se utiliza la enum
    // enum CeldaHojadeCalculo{
    //     Entero(i32),
    //     Decimal(f32),
    //     Texto(String),
    //     Booleano(bool),

    // }

    // let fila = vec![
    //     CeldaHojadeCalculo::Entero(10),
    //     CeldaHojadeCalculo::Decimal(3.14),
    //     CeldaHojadeCalculo::Texto(String::from("Hola")),
    //     CeldaHojadeCalculo::Booleano(true),
    // ];
    // for celda in fila {
    //     match celda {
    //         CeldaHojadeCalculo::Entero(valor) => println!("Entero: {}", valor),
    //         CeldaHojadeCalculo::Decimal(valor) => println!("Decimal: {}", valor),
    //         CeldaHojadeCalculo::Texto(texto) => println!("Texto: {}", texto),
    //         CeldaHojadeCalculo::Booleano(valor) => println!("Booleano: {}", valor),
    //     }
    // };

    //CADENA STRING
    // let datos = "Hola, soy un string";

    // let caena = String::from("Hola, soy un string");
    // // el "&" lo que hace es tomar referecia o presta ese valor 
    // let cadena2 = caena + &datos;

}
