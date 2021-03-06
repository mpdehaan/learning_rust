// The for in construct can be used to iterate an Iterator, a lazy value generator (more details later). One of the most common iterators is the range function. range(a, b) will yield values from a (inclusive) to b (exclusive) in steps of one.

fn main() {
    // `n` will take the values: 1, 2, ..., 100 in each iteration
    for n in range(1u, 101) {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}
