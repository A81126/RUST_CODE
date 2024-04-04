fn is_prime(num: u64) -> bool {
    if num <= 1 {
        return false;
    }

    // Check divisibility by numbers from 2 up to the square root of the number
    let mut divisor = 2;
    while divisor * divisor <= num {
        if num % divisor == 0 {
            return false;
        }
        divisor += 1;
    }

    true
}

fn main() {
    let num = 23;
    if is_prime(num) {
        println!("{} is prime", num);
    } else {
        println!("{} is not prime", num);
    }
}
