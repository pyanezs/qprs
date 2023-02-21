use ndarray::Array2;
use ndarray::Axis;

pub fn array2_to_vec(array: Array2<f64>) -> Vec<Vec<f64>> {
    array
        .axis_iter(Axis(0))
        .map(|row| row.to_owned().into_raw_vec())
        .collect()
}
