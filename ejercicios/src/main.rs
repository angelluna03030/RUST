
//Arear de un rectangulo 
struct  Area(i32, i32);
fn main() {
   let ancho1 = 15;
    let alto1 = 10;
    let area1 = area(ancho1, alto1);
    println!("El area del rectangulo es: {}", area1);
    let ancho2 = 15 ;
    let alto2 = 15 ;


println!("El area del rectangulo es: {}", areacontuplas(Area(ancho2, alto2)));
    //Solucion dos 
}


fn area(ancho: u32, alto: u32) -> u32 {
    let area = ancho * alto;
    return area;
}

fn areacontuplas(area:Area) -> i32 {
    let Area(ancho, alto) = area;
    return ancho * alto;
}