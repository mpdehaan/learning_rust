fn main() {

    let things  = vec!(1i, 2i, 3i);

    println!("The third favorite number is {}.", things[2]);

    let mut another_vector = vec!(4i);

    another_vector.push_all(&[1, 2, 3]);

    println!("The second number is {}.", another_vector[1])


}


