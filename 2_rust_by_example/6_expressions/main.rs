// statements ending in ; do not return a value.

// blocks are expressions, so they can be used as R-values in assignments.

// if the block ends in a semicolon, the value will be (), aka 'unit'.

fn main() {
    let x = 5u;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };

    let z = {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        2 * x;
    };

    println!("x is {}", x);
    println!("y is {}", y);
    println!("z is {}", z);
}
