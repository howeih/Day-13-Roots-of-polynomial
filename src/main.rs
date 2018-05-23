extern crate rulinalg;
use rulinalg::matrix::BaseMatrixMut;
use rulinalg::matrix::Matrix;

fn roots(coeffs: &[f64]) -> Vec<f64> {
	let len = coeffs.len() - 1;
	let mut matrix = Matrix::from_fn(len, len, |c, r| {
		if r == c + 1 {
			1.0
		} else {
			0.0
		}
	});
	{
		let mut col = matrix.col_mut(len - 1);
		for i in 0..len {
			col[i] = coeffs[len - i] / -coeffs[0];
		}
	}
	matrix.eigenvalues().unwrap()
}

fn main() {
	let m = [1.0, -2.0, 1.0]; // x^2 - 2x + 1 = 0
	assert_eq!(roots(&m), [1.0, 1.0]); // (x-1)(x-1)
	let m2 = [2.0, 0.0, -18.0]; //x^2-18
	assert_eq!(roots(&m2), [ -3.0, 3.0]); // (x-3)(x+3)
}
