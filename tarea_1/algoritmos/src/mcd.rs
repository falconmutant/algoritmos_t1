pub fn mcd(mut num1: i32, mut num2: i32) -> i32 {
    while num2 != 0 {
        let residue = num1 % num2;
        num1 = num2;
        num2 = residue;
    }
    num1
}