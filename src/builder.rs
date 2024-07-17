use noise::{NoiseFn, Perlin};

use crate::map::Map;

pub fn noise(width: usize, height: usize, scale: f64) -> Vec<Vec<f64>> {
    (0..height).map(|y| (0..width)
        .map(|x| {
            let sample_x = (x as f64) / scale;
            let samply_y = (y as f64) / scale;
            let perlin_value = Perlin::default();
            perlin_value.get([sample_x, samply_y])
        })
        .collect::<Vec<_>>())
        .collect::<Vec<_>>()

}

pub struct MapBuilder<M: Map> {
    map: M,
}