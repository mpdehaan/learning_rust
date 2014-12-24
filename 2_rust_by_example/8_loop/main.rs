// Rust provides a loop keyword to indicate an infinite loop.

// The break statement can be used to exit a loop at anytime, 
// whereas the continue statement can be used to skip the 
// rest of the iteration and start a new one.

fn main() {
    let mut count = 0u;

    println!("Let's count until infinity!");

    // Infinite loop
    loop {

        count += 1;

        if count == 3 {
            println!("three");
            // Skip the rest of this iteration
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");
            // Exit this loop
            break;
        }

    }

    // ============================================

    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // This would break only the inner loop
            //break;

            // This breaks the outer loop
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");


}


