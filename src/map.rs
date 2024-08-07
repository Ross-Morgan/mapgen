use ndarray::{Array, Dim, Dimension, Ix2};

use crate::coords::CartesianCoordinates;

#[derive(Clone, Debug, Default)]
pub struct TileMap<CoordComponentType, CellContentsType, Shape, const DIMENSION: usize>(
    Array<MapCell<CoordComponentType, CellContentsType, DIMENSION>, Shape>,
)
where
    CoordComponentType: Copy + Default,
    CellContentsType: MapTileContent,
    Shape: Dimension;

#[derive(Copy, Clone, Debug, Default)]
pub struct MapCell<CoordComponentType, CellContentsType, const DIMENSION: usize>(
    CartesianCoordinates<CoordComponentType, DIMENSION>,
    CellContentsType,
)
where
    CoordComponentType: Copy + Default,
    CellContentsType: MapTileContent;

impl<CoordT, ContentsT, Shape, const D: usize> TileMap<CoordT, ContentsT, Shape, D>
where
    CoordT: Copy + Default,
    ContentsT: MapTileContent,
    Shape: Dimension,
{
    pub fn new_2d(width: usize, height: usize) -> TileMap<CoordT, ContentsT, Ix2, 2> {
        let a = Array::default(Ix2(height, width));
        let map = TileMap::<CoordT, ContentsT, Ix2, 2>(a);

        map
    }
}

impl<CellContentsType: MapTileContent> ToString
    for TileMap<usize, CellContentsType, Ix2, 2>
{
    fn to_string(&self) -> String {
        let chars = self.0.iter().map(|row| row.1.to_char()).collect::<String>();
        chars
            .as_bytes()
            .chunks_exact(self.0.shape()[1])
            .map(|r| String::from_utf8(r.to_vec()).unwrap())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

pub trait MapTileContent: Default {
    fn to_char(&self) -> char;
}

pub trait Map {
    type CoordinateSystem;
}

impl<CoordT: Copy + Default, ContentT: MapTileContent, Shape: Dimension, const DIMENSION: usize> Map for TileMap<CoordT, ContentT, Shape, DIMENSION> {
    type CoordinateSystem = CartesianCoordinates<CoordT, DIMENSION>;
}