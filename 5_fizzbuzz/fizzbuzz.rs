
fn div_by_three(num: int) -> bool {
   num % 3 == 0
}

fn div_by_five(num: int) -> bool {
   num % 5 == 0
}

fn div_by_fifteen(num: int) -> bool {
   num % 5 == 0
}

#[test]
fn test_div_by_three() {
   if div_by_three(1) {
       panic!("One is not divisible by three")
   }
   if !div_by_three(3) {
       panic!("Three is divisible by three")
   }
   assert!(div_by_three(33))
}

#[test]
fn test_div_by_five() {
   if div_by_five(1) {
      panic!("1 is not divisible by five")
   }
   if !div_by_five(5) {
      panic!("5 is divisible by five")
   }
   if !div_by_five(15) {
      panic!("15 is divisible by five")
   }
}

#[test]
fn test_div_by_fifteen() {
   if div_by_fifteen(1) {
      panic!("1 is not divisible by fifteen")
   }
   if !div_by_fifteen(15) {
      panic!("15 is not divisible by fifteen")
   }
   if !div_by_fifteen(15) {
      panic!("Raargh, these tests are silly but I'm following along with the tutorial anyway")
   }
}

fn fizzbuzz(num : int) -> String {
    if div_by_fifteen(num) {
        "FizzBuzz".to_string()
    }
    else if div_by_three(num) {
        "Fizz".to_string()
    }
    else if div_by_five(num) {
        "Buzz".to_string()
    }
    else {
        "".to_string()
    }
}

fn main() {
   for num in range(1i, 100) {

      let answer = fizzbuzz(num);  
      println!("{} {}", num, answer);

   }
}

