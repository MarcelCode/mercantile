static PI: f64 = std::f64::consts::PI;
static R2D: f64 = 180.0f64 / PI;
static RE: f64 = 6378137.0;
static CE: f64 = 2.0f64 * PI * RE;
static EPSILON: f64 = 1e-14;
static LL_EPSILON: f64 = 1e-11;

#[derive(Debug)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Tile {
    fn new(x: i32, y: i32, z: i32) -> Self {
        let (lo, hi) = minmax(z);
        if !(lo <= x && x <= hi) || !(lo <= y && y <= hi) {
            panic!("require tile x and y to be within the range (0, 2 ** zoom)");
        }
        Tile { x, y, z }
    }
}

fn minmax(z: i32) -> (i32, i32) {
    let max_value = 2_i32.pow(z as u32);
    (0, max_value - 1)
}

#[derive(Debug, PartialEq)]
pub struct LngLat {
    pub lng: f64,
    pub lat: f64,
}

#[derive(Debug, PartialEq)]
pub struct LngLatBbox {
    pub west: f64,
    pub south: f64,
    pub east: f64,
    pub north: f64,
}

#[derive(Debug, PartialEq)]
pub struct Bbox {
    pub left: f64,
    pub bottom: f64,
    pub right: f64,
    pub top: f64,
}

pub fn ul(tile: Tile) -> LngLat {
    let z2 = 2.0_f64.powf(tile.z as f64);
    let lon_deg = tile.x as f64 / z2 * 360.0 - 180.0;
    let lat_rad = (PI * (1.0 - 2.0 * tile.y as f64 / z2)).sinh().atan();
    let lat_deg = lat_rad.to_degrees();

    LngLat { lng: lon_deg, lat: lat_deg }
}

pub fn bounds(tile: Tile) -> LngLatBbox {
    let z2 = 2.0_f64.powf(tile.z as f64);

    let ul_lon_deg = tile.x as f64 / z2 * 360.0 - 180.0;
    let ul_lat_rad = (PI * (1.0 - 2.0 * tile.y as f64 / z2)).sinh().atan();
    let ul_lat_deg = ul_lat_rad.to_degrees();

    let lr_lon_deg = (tile.x + 1) as f64 / z2 * 360.0 - 180.0;
    let lr_lat_rad = (PI * (1.0 - 2.0 * (tile.y + 1) as f64 / z2)).sinh().atan();
    let lr_lat_deg = lr_lat_rad.to_degrees();

    LngLatBbox { west: ul_lon_deg, south: lr_lat_deg, east: lr_lon_deg, north: ul_lat_deg }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ul() {
        let tile: Tile = Tile { x: 486, y: 332, z: 10 };
        let result = ul(tile);
        let expected_result = LngLat { lng: -9.140625, lat: 53.33087298301705 };

        assert_eq!(result, expected_result)
    }

    #[test]
    fn test_bounds() {
        let tile: Tile = Tile { x: 486, y: 332, z: 10 };
        let result = bounds(tile);
        let expected_result = LngLatBbox {
            west: -9.140625,
            south: 53.120405283106564,
            east: -8.7890625,
            north: 53.33087298301705,
        };

        assert_eq!(result, expected_result)
    }
}
