fn main() {

    // for num in range(0u, 500) {
    //    println!("Hello");
    // }

    // for num in range(0u, 500) {
    //    spawn(move || 
    //        println!("Hello")
    //    )
    // }

    let (chan, port) = channel();

    spawn(move ||
        chan.send(10u)
    );

    println!("{}", port.recv().to_string());

}
