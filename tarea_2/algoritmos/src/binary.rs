pub fn sum(a: &Vec<i32>, b: &Vec<i32>) -> Vec<i32> {
    let n = a.len();

    let mut result = vec![0; n + 1];
    let mut carry = 0;

    for i in (0..n).rev() {
        let sum = a[i] + b[i] + carry;

        result[i + 1] = sum % 2;
        carry = sum / 2;
    }

    result[0] = carry;

    result
}