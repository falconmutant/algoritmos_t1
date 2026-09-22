pub fn is_prime(number: i32) -> bool {
    if number < 2 {
        return false;
    }

    for i in 2..=(number as f64).sqrt() as i32 {
        if number % i == 0 {
            return false;
        }
    }
    true
}
