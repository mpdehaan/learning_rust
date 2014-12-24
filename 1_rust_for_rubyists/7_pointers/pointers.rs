// A box, with type signature Box<T>, is a smart pointer to a heap allocated value of type T
// source: http://rustbyexample.com/box.html

fn plus_one(x: Box<int>) -> int {
   *x + 1
}

fn main() {

    let y = box 10i;

    println!("{}", plus_one(y));

}
