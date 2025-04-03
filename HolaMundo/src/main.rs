// struct Usuario {
//     nombre: String,
//     edad: u8,
//     email: String,
//     activo: bool,
// }
struct  Color (i32, i32, i32);
struct Coordenada(i32, i32, i32);
fn main() {

 
    //es muy importa que si se define una estrucutra mutable todas susu propiedades
    //se vuelven mutables
//     let mut usuario1 = Usuario {
//         activo: true,
//         edad: 12,
//         email: String::from("Angel@gmail.com"),
//         nombre: String::from("Angel"),
//     };
//    usuario1.nombre= String::from("Angelito");
//     println!("Nombre: {}", usuario1.nombre);
//     if usuario1.edad >= 18 {
//         println!("es adulto ");
//     } else {
//         println!("es menor de edad");
//     }

//     let usuario2 = nuevo_usuario(String::from("Camilo"), 
//     String::from("camilo@gmail.com"));

//     println!("Edad : {}", usuario2.edad);

//  //aqui le estanos agregando la informacion restante que he mos sacado del usuairio
//     let usuario3 = Usuario {
//         nombre: String::from("Juan"),
//         ..usuario2 // Copia los valores de usuario2
//     };
//     //va a silir la misma informacion que el usuario2

//     println!("Nombre: {}", usuario3.email);


    //Estructura de tuplas
    let negro = Color(0,0,0);
    let cordenada = Coordenada(0,0,9);

//Estructura de tipo unidad 

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
    // for numero in 1..20 {
    //     println!("valor: {}", numero);
    // }
    //--> PROPIEDAD
    // {let i = 0;}
    //     let cadena = String::from("Hola");
    //     println!("cadena: {}", cadena);

    // let i:i32 = 7;
    // let j:i32 = i;
    // println!("i: {}", i);
    // println!("j: {}", j);
    // importate cunande reasigno un valora otra varible el valor
    // anterior (valor 1)dejar ir a el espacio en memoria, para poder volverlo a utlizar
    //  */

    // let cadena= String::from("hol a");
    // // !! ** debemosde clonar el valor el valor 1 **
    // let cadanea2 = cadena.clone();
    // println!("cadena: {}", cadena);
    //  println!("cadena2: {}", cadanea2);

    //REFERENCIA y PRÉSTAMO
    // al ser colocada una varible a un funcion la varible toma la propiedad por defecto
    //     let s = String::from("hola Amigo");
    //     let (y, longitub) = toma_propiedad(s);
    //     println!("s: {}", y);

    // println!("longitud: {}", longitub);

    // let z = String::from("Hola");

    // let longitub2 = valor_entrero(&z);
    // println!("longitud: {}", longitub2);

    //  let mut  t = String::from("Hola Word ");
    //  modficar(&mut t);
    //  println!( "longitud: {}", t)

    //let mut s = String::from("Hola ");
    // modficar(&mut s);
    // println!("s: {}", s);

    /*y porque pasa esto con las restricicioines en cuanto a los prestamos,
    porque ya con el primer print se acaba el ambieto de cad1 y cad2
     (ose se acabo su uso)*/
    // let cad1 = &s;
    // let cad2 = &s;
    // println!("cad1: {} {} ", cad1, cad2);
    // let cad3 = &mut s;

    // println!("cad1: {}", cad3);

    // let referencia = colgante();

    // println!("referencia: {}", referencia);

    // // Registrar el tiempo inicial
    // let start_time = Instant::now();

    // // Ejecutar el bucle 100,000 veces
    // for _ in 0..10_000 {
    //     println!("Hola Mundo");
    // }

    // // Calcular y mostrar la duración
    // let duration = start_time.elapsed();
    // println!("El tiempo de ejecución fue de {:?} segundos", duration.as_secs_f64());

    //segmento
    // let cadena = String::from("hola amigos del rust");
    // cuando es inico de la cadena es 0 funciona solo colorcar solo dos puntos
    // y lo mismo podemos hacer con el fin.
    // let hola = &cadena[..4];
    // let amigos = &cadena[5..11];
    // println!("hola: {} {}", hola, amigos);
    // println!("primera palabra: {}", primera_palabra(&cadena));

    //     let n = [1,2,3,4,5,6];
    //     let segmento = &n[1..3];
    //   assert_eq!(segmento,[2,3])

    //ESTRUCTURA
}
// fn primera_palabra(cadena: &String) -> &str {
//     let bytes = cadena.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &cadena[..i];
//         }
//     }
//     &cadena[..]
// }

// fn colgante() -> &'static str {
//     let s = "hola referecia ";
//     s
// }
//en la funciones en rust puede retornar mas de un valor.
// fn toma_propiedad(cadena: String)->  (String , usize){
//     let aux:usize = cadena.len();
//     println!("{}", cadena);
//     (cadena , aux)
// }

// //aqui se esta creando una funcion en donde el "&" es que una
// // valiable a la que se va a hacer eferecia pero no devolver.
// fn valor_entrero(cadena: &String)->usize{
// let aux:usize = cadena.len();
//     println!("{}", cadena);
//     aux
// }
//no se puede mofdificar una varible que se piede prestado.
// se pede modificar una varible cuando le colocamo mut
// fn modficar(cadena: &mut String) {
//     cadena.push_str("Amigos ");
// }
// fn nuevo_usuario(nombre: String, email: String) -> Usuario {
//     Usuario {
//         nombre,
//         edad: 0,
//         email,
//         activo: false,
//     }
// }
