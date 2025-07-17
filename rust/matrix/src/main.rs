use crate::matrix::Matrix;

mod matrix;

#[allow(unused)]
fn main() {
    let mut my_matrix: Matrix<i32> = Matrix::new();
    let my_row = vec![1, 3, 3, 7];
    my_matrix.add_row(my_row);
    let my_row = vec![9, 14, 64, 96];
    my_matrix.add_row(my_row);
    let my_col = vec![22, 44];
    my_matrix.add_col(my_col);
    println!("{my_matrix:?}");
    my_matrix.transpose();
    println!("{my_matrix:?}");
}
