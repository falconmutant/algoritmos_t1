pub fn transpose(values: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let rows = values.len();
    let columns = values[0].len();
    let mut transposed = vec![vec![0; rows]; columns];

    for i in 0..rows {
        for j in 0..columns {
            transposed[j][i] = values[i][j];
        }
    }
    transposed
}

pub fn flip(values: &mut Vec<Vec<i32>>) -> &Vec<Vec<i32>> {
    let rows = values.len();
    let columns = values[0].len();
    let total = rows * columns;

    for k in 0..total / 2 {
        let mirror = total - 1 - k;
        let i = k / columns;
        let j = k % columns;
        let row = mirror / columns;
        let column = mirror % columns;
        let temp = values[i][j];

        values[i][j] = values[row][column];
        values[row][column] = temp;
    }
    values
}

pub fn major(values: &Vec<Vec<i32>>) -> i32 {
    let mut max = values[0][0];

    for i in 0..values.len() {
        for j in 0..values[i].len() {
            if values[i][j] > max {
                max = values[i][j];
            }
        }
    }

    max
}