//el Option sirve para tener un valor opcional y el el valor como tal  
/*
pub enum Option<T> {
    None,// no puede conteenr nada 
    Some(Valor), // puede contener un valor,
}
*/
fn incrementar_uno(x: Option<i32>) -> Option<i32> {
    //match sirve para el control de flujo ose para que verificar que todo esta bien definido
    // o para que cunado este verificado algo se pueda ejecutar el proceso
    match x {
        Some(valor) => Some(valor + 1),
        None => None,
    }
}
fn main() {
    let valor = Some(5);
    let resultado = incrementar_uno(valor);
    println!("{:?}", resultado); // Imprime Some(6)

    let valor_nulo: Option<i32> = Some(5);
    let resultado_nulo = incrementar_uno(valor_nulo);
    println!("{:?}", resultado_nulo); // Imprime None
}
