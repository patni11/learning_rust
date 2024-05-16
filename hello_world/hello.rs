// fn main(){
//     println!("Hello World");
//     println!("I am rustecan");
// }

fn main2(){
    println!("Hello World");
}

// Fix the error below with least amount of modification to the code
fn main() {
    let x: i32 = 5; // Uninitialized but used, ERROR !
    let y: i32; // Uninitialized but also unused, only a Warning !

    assert_eq!(x, 5);
    println!("Success!");
    main2();
}


