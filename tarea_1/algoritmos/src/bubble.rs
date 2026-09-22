pub fn sort(values: &mut Vec<i32>) -> &Vec<i32> {
    for i in 0..values.len() - 1 {
        for j in 0..values.len() - 1 - i {
            if values[j] > values[j + 1] {
                let temp = values[j];
                values[j] = values[j + 1];
                values[j + 1] = temp;
            }
        }
    }
    values
}