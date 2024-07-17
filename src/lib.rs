pub mod builder;
pub mod coords;
pub mod map;

pub mod prelude {
    use super::*;

    pub use builder::MapBuilder;
    pub use coords::{CartesianCoordinates, CoordinateSystem, GenericCoordinates};
    pub use map::{Map, MapCell, MapTileContent, TileMap};
}
