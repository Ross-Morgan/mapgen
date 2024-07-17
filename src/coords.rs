pub enum GenericCoordinates<T, const N: usize> {
    // Hexagonal(HexagonalCoordinates<T>),
    Square(CartesianCoordinates<T, N>),
}

// pub enum HexagonalCoordinates<T> {
//     Offset {
//         rows_shifted: bool,
//         odd_shifted: bool,
//         row: T,
//         col: T,
//     },
//     Cube(T, T, T),
//     Axial
// }

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CartesianCoordinates<T, const N: usize>([T; N]);

impl<T, const N: usize> CartesianCoordinates<T, N> {
    pub const fn new(components: [T; N]) -> Self {
        Self(components)
    }
}

impl<const N: usize> CartesianCoordinates<f64, N> {
    pub fn distance_from(&self, other: &CartesianCoordinates<f64, N>) -> f64 {
        self.0
            .iter()
            .copied()
            .zip(other.0.iter().copied())
            .fold(0.0, |sum, (l, r)| sum + (r - l).powi(2))
            .sqrt()
    }

    pub fn modulus(&self) -> f64 {
        self.0
            .iter()
            .copied()
            .fold(0.0, |sum, v| sum + v.powi(2))
            .sqrt()
    }
}

impl<T: Copy + Default, const N: usize> Default for CartesianCoordinates<T, N> {
    fn default() -> Self {
        Self([T::default(); N])
    }
}

pub trait CoordinateSystem: Copy + Default {}

impl<T: Copy + Default, const N: usize> CoordinateSystem for CartesianCoordinates<T, N> {}