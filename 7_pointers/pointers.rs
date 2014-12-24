fn main() {

    // box creates an 'owned' pointer, there can be only one
    let x = box 10i;

    // this makes a copy of the number 10, it's not the same one.
    let y = x.clone();

    println!("{}", *x);

}
