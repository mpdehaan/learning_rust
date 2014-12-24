
// This function only gets compiled if the target OS is linux
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!")
}

// And this function only gets compiled if the target OS is *not* linux
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running linux!")
}

#[cfg(some_condition)]
fn conditional_function() {
    println!("condition met!")
}

fn main() {
    are_you_on_linux();
    conditional_function();
}
