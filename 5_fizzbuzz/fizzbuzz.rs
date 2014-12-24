
fn div_by_three(num: int) -> bool {
   num % 3 == 0
}

#[test]
fn test_div_by_three() {
   if div_by_three(1) {
       panic!("One is not divisible by three")
   }
   if !div_by_three(3) {
       panic!("Three is divisible by three")
   }
}

fn main() {
   for num in range(1i, 100) {
      println!("{}", num)
   }
}

