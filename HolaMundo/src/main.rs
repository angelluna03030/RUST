fn main() {
    //Se defiene varibles mutable  o que se pueden cambiar pero tiene que hacer 
    // del mismo tio 
    let mut x:i32 = 7;

    println!("Hello, amigos! x = {}", x);
    x = 42;
    println!("Hello, amigos! x = {}", x);
    //Se define una variable inmutable o que no se puede cambiar
    const PI: f32 = 3.14159;
    println!("Hello, amigos! PI = {}", PI);

    // Se define una variable que se puede cambiar, de la siguente manera. 
    let y = 3;
    println!("Hello, amigos! y = {}", y);
    let Y = "3";
    println!("Hello, amigos! Y = {}", Y);
}
