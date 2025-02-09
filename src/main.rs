use math_lib::linalg::matrix::Matrix;

fn main() {
    let a = Matrix::new((1..=25).map(|x| x as f64).collect(), 5, 5);
    println!("{}", a.det());
}
