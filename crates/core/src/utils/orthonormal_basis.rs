use crate::Vector3;

/// An orthonormal basis constructed from a normal vector.
///
/// An orthonormal basis is a set of three mutually perpendicular unit vectors
/// that form a coordinate system. This is useful for transforming vectors between
/// different coordinate spaces, such as converting from world space to tangent space.
///
/// The basis is constructed using the Gram-Schmidt process, ensuring:
/// - All three vectors (u, v, w) are unit length
/// - All three vectors are mutually perpendicular
/// - The vectors form a right-handed coordinate system
///
/// # Examples
///
/// ```
/// use rust_raytracer_core::{utils::OrthonormalBasis, Vector3};
///
/// let normal = Vector3::new(0.0, 1.0, 0.0);
/// let basis = OrthonormalBasis::new(normal);
///
/// // Transform a vector to local space
/// let world_vec = Vector3::new(1.0, 0.0, 0.0);
/// let local_vec = basis.transform_to_local(world_vec);
/// ```
pub struct OrthonormalBasis {
    /// The first basis vector
    pub u: Vector3,
    /// The second basis vector
    pub v: Vector3,
    /// The third basis vector (normal direction)
    pub w: Vector3,
}

impl OrthonormalBasis {
    /// Constructs a new orthonormal basis from a normal vector.
    ///
    /// The normal vector is normalized to become the `w` basis vector. The `u` and `v`
    /// vectors are then constructed to be perpendicular to `w` and to each other using
    /// the Gram-Schmidt process.
    ///
    /// The algorithm chooses an initial reference vector that is not parallel to the
    /// normal: (0, 1, 0) if the normal is mostly aligned with the x-axis, otherwise (1, 0, 0).
    ///
    /// # Arguments
    ///
    /// * `normal` - The normal vector to build the basis from. Does not need to be normalized.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_raytracer_core::{utils::OrthonormalBasis, Vector3};
    ///
    /// // Create basis from upward-pointing normal
    /// let basis = OrthonormalBasis::new(Vector3::new(0.0, 1.0, 0.0));
    /// ```
    pub fn new(normal: Vector3) -> Self {
        let w = normal.unit();
        let a = if w.x.abs() > 0.9 {
            Vector3::new(0.0, 1.0, 0.0)
        } else {
            Vector3::new(1.0, 0.0, 0.0)
        };
        let v = w.cross(&a).unit();
        let u = w.cross(&v);

        Self { u, v, w }
    }

    /// Transforms a vector from basis coordinates to local space.
    ///
    /// Given a vector in the basis coordinate system (where the basis vectors are the axes),
    /// this function returns the equivalent vector in local/world space.
    ///
    /// The transformation is computed as: `v.x * u + v.y * v + v.z * w`
    ///
    /// # Arguments
    ///
    /// * `v` - A vector in basis coordinates
    ///
    /// # Returns
    ///
    /// The vector transformed to local space
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_raytracer_core::{utils::OrthonormalBasis, Vector3};
    ///
    /// let basis = OrthonormalBasis::new(Vector3::new(0.0, 1.0, 0.0));
    /// let basis_vec = Vector3::new(1.0, 0.0, 0.0); // Unit vector along basis u-axis
    /// let local_vec = basis.transform_to_local(basis_vec);
    /// // local_vec is now aligned with the u basis vector in world space
    /// ```
    pub fn transform_to_local(&self, v: Vector3) -> Vector3 {
        (v.x * self.u) + (v.y * self.v) + (v.z * self.w)
    }
}
