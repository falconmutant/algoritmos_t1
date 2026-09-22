pub fn sort_ascending(values: &mut Vec<i32>) -> &Vec<i32> {
    for i in 1..values.len() {
        let key = values[i];
        let mut j = i;

        while j > 0 && values[j - 1] > key {
            values[j] = values[j - 1];
            j -= 1;
        }
        values[j] = key;
    }
    values
}

pub fn sort_descending(values: &mut Vec<i32>) -> &Vec<i32> {
    for i in 1..values.len() {
        let key = values[i];
        let mut j = i;

        while j > 0 && values[j - 1] < key {
            values[j] = values[j - 1];
            j -= 1;
        }
        values[j] = key;
    }
    values
}