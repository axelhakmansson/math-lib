use math_lib::linalg::matrix::Matrix;

fn main() {
    let a = Matrix::new((1..=4).map(|x| x as f64).collect(), 2, 2);
    println!("{}", a.inverse());
}
