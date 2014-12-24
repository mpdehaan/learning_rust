fn plus_one(x: Box<int>) -> int {
   *x + 1
}

fn main() {

    let y = box 10i;

    println!("{}", plus_one(y));

}
