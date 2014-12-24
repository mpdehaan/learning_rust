fn main() {

    // for num in range(0u, 500) {
    //    println!("Hello");
    // }

    for num in range(0u, 500) {
        spawn(move || 
            println!("Hello")
        )
    }

}
