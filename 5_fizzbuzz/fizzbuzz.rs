
fn div_by_three(num: int) -> bool {
   if num % 3 == 0 {
      true
   } else {
      false
   }
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

