use std::time::Instant;

const N: usize = 4;

type Array2D = [[i8; N]; N];

fn main() {
    let array: Array2D = [[1i8; N]; N];

    let start_time = Instant::now();

    let res1 = sum_row(&array);

    let t1 = start_time.elapsed();

    let res2 = sum_col(&array);

    let t2 = start_time.elapsed();

    println!("{} with {:?}", res1, t1);
    println!("{} with {:?}", res2, t2 - t1);
}

fn sum_row(matrix: &Array2D) -> i8 {
    let mut result = 0i8;

    let mut x = 0;
    while x < N * N {
        let row = x / N;
        let col = x % N;

        result +=
            matrix[row][col] + matrix[row][col + 1] + matrix[row][col + 2] + matrix[row][col + 3];

        x += 4;
    }

    result
}

fn sum_col(matrix: &Array2D) -> i8 {
    let mut result = 0i8;

    let mut x = 0;
    while x < N * N {
        let row = x % N;
        let col = x / N;

        result +=
            matrix[row][col] + matrix[row + 1][col] + matrix[row + 2][col] + matrix[row + 3][col];

        x += 4;
    }

    result
}