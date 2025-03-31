fn main() {
 let suma2 = suma(3,4);
 println!("La suma es: {}", suma2);
saludo("Juan Carlos");
println!("El número es: {}", trece());
}



fn suma (x: i32, y: i32) -> i32 {
    println!("Hola desde la función mifuncion");
    return  x + y;
}

/// Prints a greeting message with the given name.
///
/// # Arguments
///
/// * `name` - A string slice that holds the name to be greeted.
///
/// # Examples
///
/// ```
/// saludo("Alice");
/// ```
fn saludo (name: &str) {
    println!("Hola mi nombre es: {}", name);
}fn trece ()->i8{
    return  13;
}