// fn insertion_sort<T: Ord>(arr: &mut [T]) {
//     for i in 1..arr.len() {
//         let mut j = i;
//         while j > 0 && arr[j] < arr[j - 1] {
//             arr.swap(j, j - 1);
//             j -= 1;
//         }
//     }
// }
// pub fn insertion_sort<T: Ord>(arr: &mut [T]) { /* Definición aquí */ }

// pub fn merge<T: Ord>(arr: &mut [T], left: usize, mid: usize, right: usize) { /* Definición aquí */ }

// pub fn timsort<T: Ord>(arr: &mut [T]) { /* Definición aquí */ }
// fn merge<T: Ord>(arr: &mut [T], left: usize, mid: usize, right: usize) {
//     let mut left_part = arr[left..mid].to_vec();
//     let mut right_part = arr[mid..right].to_vec();

//     let mut i = 0;
//     let mut j = 0;
//     for k in left..right {
//         if i < left_part.len() && (j >= right_part.len() || left_part[i] <= right_part[j]) {
//             arr[k] = left_part[i].clone();
//             i += 1;
//         } else {
//             arr[k] = right_part[j].clone();
//             j += 1;
//         }
//     }
// }

// fn timsort<T: Ord>(arr: &mut [T]) {
//     let run_size = 32;

//     for i in (0..arr.len()).step_by(run_size) {
//         let end = std::cmp::min(i + run_size, arr.len());
//         insertion_sort(&mut arr[i..end]);
//     }

//     let mut size = run_size;
//     while size < arr.len() {
//         let mut left = 0;
//         while left + size < arr.len() {
//             let mid = left + size;
//             let right = std::cmp::min(left + 2 * size, arr.len());
//             merge(arr, left, mid, right);
//             left += 2 * size;
//         }
//         size *= 2;
//     }
// }
// fn main() {
//     let mut data = vec![5, 2, 9, 1, 5, 6];
//     timsort(&mut data);
//     println!("{:?}", data);
// }

use std::env;

fn main() {
    let argumentos: Vec<String> = env::args().collect();
    let paramietro1 = &argumentos[0];
    let paramietro2 = &argumentos[1];

    println!("EL primer parametro es {}", paramietro1);
    println!("EL segundo parametro es {}", paramietro2);
}
// }
// #[cfg(test)]
// mod test{
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
