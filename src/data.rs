#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Orientation {
    /// # Description
    /// Something is oriented horizontally
    ///
    /// # Example
    /// ```
    /// --------
    /// ```
    Horizontal,

    /// # Description
    /// Something is oriented vertically
    ///
    /// # Example
    /// ```
    /// |
    /// |
    /// |
    /// |
    /// ```
    Vertical,
}
#[derive(Clone, Copy, Debug)]
pub struct Rectangle<T> {
    pub x: T,
    pub y: T,
    pub width: T,
    pub height: T,
}
#[derive(Clone, Copy, Debug)]
pub struct Size<T> {
    pub width: T,
    pub height: T,
}