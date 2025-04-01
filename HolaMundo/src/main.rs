fn main() {
    // let mut contador:i32 = 5;
    // let restulado = loop {
    //     contador += 1;
    //     if contador >= 10 {
    //         break contador * contador; // Return the value of contador when breaking
    //     }
    //     println!("contador: {}", contador);
    // };
    // println!("resultado: {}", restulado);
    // let matriz = [12, 23, 2, 3, 24];
    // let mut i = 0;
    // while i < matriz.len() {
    //     println!("valor: {}", matriz[i]);
    //     i += 1;
    // };

    /*
    El método  crea un iterador que permite recorrer los elementos de la colección  sin consumirla. Esto significa que la matriz original no se modifica durante la iteración.
    Cada elemento de matriz se toma prestado (borrowed) de manera inmutable dentro del bucle.

    */
    // for numero in matriz.iter() {
    //     println!("valor: {}", numero);
    // }
    /*para crear un rango en rust*/
    for numero in 1..20 {
        println!("valor: {}", numero);
    }
}
