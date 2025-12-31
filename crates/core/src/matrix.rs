use crate::Vector3;
use std::ops::{Index, Mul};

/// A 3x3 matrix for linear transformations in 3D space.
///
/// This structure represents a 3x3 matrix stored in row-major order,
/// commonly used for rotations, scaling, and other linear transformations
/// of 3D vectors.
///
/// # Examples
///
/// ```
/// use caustic_core::Matrix3x3;
///
/// let identity = Matrix3x3::new([
///     [1.0, 0.0, 0.0],
///     [0.0, 1.0, 0.0],
///     [0.0, 0.0, 1.0],
/// ]);
/// ```
#[derive(Debug)]
pub struct Matrix3x3 {
    /// Internal storage for the 3x3 matrix in row-major order.
    /// `matrix[row][col]` accesses the element at the given row and column.
    matrix: [[f64; 3]; 3],
}

impl Matrix3x3 {
    /// Creates a new 3x3 matrix from a 2D array.
    ///
    /// # Arguments
    ///
    /// * `matrix` - A 3x3 array of `f64` values in row-major order
    ///
    /// # Examples
    ///
    /// ```
    /// use caustic_core::Matrix3x3;
    ///
    /// let rotation_z = Matrix3x3::new([
    ///     [0.0, -1.0, 0.0],
    ///     [1.0,  0.0, 0.0],
    ///     [0.0,  0.0, 1.0],
    /// ]);
    /// ```
    pub fn new(matrix: [[f64; 3]; 3]) -> Self {
        Self { matrix }
    }
}

/// Allows indexing into the matrix to access rows.
///
/// # Examples
///
/// ```
/// use caustic_core::Matrix3x3;
///
/// let m = Matrix3x3::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
/// let first_row = m[0]; // Returns [1.0, 2.0, 3.0]
/// let element = m[1][2]; // Returns 6.0
/// ```
impl Index<usize> for Matrix3x3 {
    type Output = [f64; 3];

    fn index(&self, index: usize) -> &Self::Output {
        &self.matrix[index]
    }
}

/// Implements matrix-vector multiplication for a 3x3 matrix and a 3D vector.
///
/// Computes the product `M * v` where `M` is a 3x3 matrix and `v` is a Vector3.
/// The result is a new Vector3 representing the transformed vector.
///
/// # Examples
///
/// ```
/// use caustic_core::{Matrix3x3,Vector3};
///
/// let matrix = Matrix3x3::new([
///     [2.0, 0.0, 0.0],
///     [0.0, 2.0, 0.0],
///     [0.0, 0.0, 2.0],
/// ]);
/// let v = Vector3::new(1.0, 2.0, 3.0);
/// let result = &matrix * v; // Vector3::new(2.0, 4.0, 6.0)
/// ```
impl Mul<Vector3> for &Matrix3x3 {
    type Output = Vector3;

    fn mul(self, v: Vector3) -> Self::Output {
        Vector3::new(
            self.matrix[0][0] * v.x + self.matrix[0][1] * v.y + self.matrix[0][2] * v.z,
            self.matrix[1][0] * v.x + self.matrix[1][1] * v.y + self.matrix[1][2] * v.z,
            self.matrix[2][0] * v.x + self.matrix[2][1] * v.y + self.matrix[2][2] * v.z,
        )
    }
}
