fn main() {

    // box creates an 'owned' pointer, there can be only one
    let x = box 10i;
    // we can't do: let y = x;
    let y = x;
    println!("{}", *x);

}
