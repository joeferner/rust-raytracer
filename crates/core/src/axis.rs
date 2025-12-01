/// Represents the three primary spatial axes in 3D space.
///
/// This enum is used to specify directional components along the X, Y, or Z axis.
///
/// # Examples
///
/// ```
/// use rust_raytracer_core::Axis;
///
/// let axis = Axis::X;
/// println!("Selected axis: {:?}", axis);
///
/// // Iterate over all axes
/// for axis in Axis::iter() {
///     println!("{:?}", axis);
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    /// The X-axis (horizontal, left-right)
    X,
    /// The Y-axis (vertical, up-down)
    Y,
    /// The Z-axis (depth, forward-backward)
    Z,
}

impl Axis {
    /// Returns an iterator over all three axes in order: X, Y, Z.
    ///
    /// This is useful when you need to perform operations on all axes,
    /// such as transforming coordinates or checking bounds in all directions.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_raytracer_core::Axis;
    ///
    /// let axes: Vec<Axis> = Axis::iter().collect();
    /// assert_eq!(axes, vec![Axis::X, Axis::Y, Axis::Z]);
    /// ```
    ///
    /// # Performance
    ///
    /// This iterator is very efficient as it simply iterates over a static array.
    pub fn iter() -> impl Iterator<Item = Axis> {
        static AXES: [Axis; 3] = [Axis::X, Axis::Y, Axis::Z];
        AXES.iter().copied() // .copied() is used to iterate over values, not references
    }
}
