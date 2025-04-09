fn mayor_i32(lista: &[i32]) -> i32 {
let mut mayor = lista[0];
   for &item in lista {
    if item > mayor {
        mayor = item;
    }
   }
   mayor
}


fn mayor_char(list: &[char])->char {
    let mut mayor = list[0];
    for &item in list {
        if item > mayor {
            mayor = item;
        }
    }
    mayor
}
fn main() {

    let lista = vec![1, 2, 3, 4, 5];
    let resultado = mayor_i32(&lista);
    println!("El mayor es: {}", resultado);

    //TTPOS DE DATOS GENERICOS 

}
