//! # Mercantile
//!
//! This module provides utility functions for working with geographical coordinates and tiles.
//! It includes functions for converting between latitude/longitude (LngLat) coordinates and
//! web mercator projected coordinates (XY), as well as functions for working with map tiles
//! and their bounding boxes.
//!
//! # Examples
//!
//! ```
//! use mercantile::*;
//!
//! // Create a new tile object
//! let tile = Tile::new(486, 332, 10);
//!
//! // Calculate the upper-left geographical coordinates of the tile
//! let ul_coordinates = ul(tile);
//! println!("Upper-left coordinates: {:?}", ul_coordinates);
//!
//! // Calculate the bounding box of the tile in geographical coordinates
//! let bbox = bounds(tile);
//! println!("Bounding box: {:?}", bbox);
//!
//! // Calculate the bounding box of the tile in web mercator coordinates
//! let xy_bbox = xy_bounds(tile);
//! println!("XY Bounding box: {:?}", xy_bbox);
//!
//! // Convert LngLat coordinates to XY coordinates
//! let lng_lat = LngLat { lng: -9.140625, lat: 53.33087298301705 };
//! let xy_coordinates = convert_xy(lng_lat);
//! println!("XY coordinates: {:?}", xy_coordinates);
//!
//! // Convert XY coordinates back to LngLat coordinates
//! let xy_coordinates = XY { x: -1017529.7205322663, y: 7044436.526761846 };
//! let lng_lat = convert_lng_lat(xy_coordinates);
//! println!("LngLat coordinates: {:?}", lng_lat);
//!
//!
//!  // Get neighbor tiles
//! let neighbors = get_neighbors(tile);
//! println!("Neighbor tiles: {:?}", neighbors);
//! ```
//!

/// Constant representing the value of PI
const PI: f64 = std::f64::consts::PI;

/// Constant representing the conversion factor from radians to degrees
const R2D: f64 = 180.0f64 / PI;

/// Earth radius in meters
const RE: f64 = 6378137.0;

/// Circumference of the Earth
const CE: f64 = 2.0f64 * PI * RE;

/// Represents a tile on a map
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Tile {
    /// Creates a new Tile object with the specified x, y, and zoom level
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        let (lo, hi) = minmax(z);
        if !(lo <= x && x <= hi) || !(lo <= y && y <= hi) {
            panic!("require tile x and y to be within the range (0, 2 ** zoom)");
        }
        Tile { x, y, z }
    }
}

/// Returns the minimum and maximum values for a tile at the given zoom level
fn minmax(z: i32) -> (i32, i32) {
    let max_value = 2_i32.pow(z as u32);
    (0, max_value - 1)
}

/// Represents a geographical coordinate in longitude and latitude
#[derive(Debug, PartialEq)]
pub struct LngLat {
    pub lng: f64,
    pub lat: f64,
}

/// Represents a point in web mercator projected coordinates
#[derive(Debug, PartialEq)]
pub struct XY {
    pub x: f64,
    pub y: f64,
}

/// Represents a bounding box in geographical coordinates
#[derive(Debug, PartialEq)]
pub struct LngLatBbox {
    /// Westernmost longitude of the bounding box
    pub west: f64,
    /// Southernmost latitude of the bounding box
    pub south: f64,
    /// Easternmost longitude of the bounding box
    pub east: f64,
    /// Northernmost latitude of the bounding box
    pub north: f64,
}

/// Represents a bounding box in web mercator projected coordinates
#[derive(Debug, PartialEq)]
pub struct Bbox {
    /// Left coordinate of the bounding box
    pub left: f64,
    /// Bottom coordinate of the bounding box
    pub bottom: f64,
    /// Right coordinate of the bounding box
    pub right: f64,
    /// Top coordinate of the bounding box
    pub top: f64,
}

/// Calculates the upper-left geographical coordinates of a given tile
pub fn ul(tile: Tile) -> LngLat {
    let z2 = 2.0_f64.powf(tile.z as f64);
    let lon_deg = tile.x as f64 / z2 * 360.0 - 180.0;
    let lat_rad = (PI * (1.0 - 2.0 * tile.y as f64 / z2)).sinh().atan();
    let lat_deg = lat_rad.to_degrees();

    LngLat { lng: lon_deg, lat: lat_deg }
}

/// Calculates the bounding box of a given tile in geographical coordinates
pub fn bounds(tile: Tile) -> LngLatBbox {
    let z2 = 2.0_f64.powf(tile.z as f64);

    let west = tile.x as f64 / z2 * 360.0 - 180.0;
    let north_rad = (PI * (1.0 - 2.0 * tile.y as f64 / z2)).sinh().atan();
    let north = north_rad.to_degrees();

    let east = (tile.x + 1) as f64 / z2 * 360.0 - 180.0;
    let south_rad = (PI * (1.0 - 2.0 * (tile.y + 1) as f64 / z2)).sinh().atan();
    let south = south_rad.to_degrees();

    LngLatBbox { west, south, east, north }
}

/// Calculates the bounding box of a given tile in web mercator projected coordinates
pub fn xy_bounds(tile: Tile) -> Bbox {
    let tile_size = CE / 2.0_f64.powf(tile.z as f64);
    let left = tile.x as f64 * tile_size - CE / 2.0;
    let right = left + tile_size;
    let top = CE / 2.0 - tile.y as f64 * tile_size;
    let bottom = top - tile_size;

    Bbox { left, bottom, right, top }
}

/// Converts geographical coordinates (LngLat) to web mercator projected coordinates (XY)
pub fn convert_xy(lng_lat: LngLat) -> XY {
    let x = RE * lng_lat.lng.to_radians();

    let y: f64;
    if lng_lat.lat <= -90.0 {
        y = f64::NEG_INFINITY;
    } else if lng_lat.lat >= 90.0 {
        y = f64::INFINITY;
    } else {
        y = RE * ((PI * 0.25) + (0.5 * lng_lat.lat.to_radians())).tan().ln();
    }

    XY { x, y }
}

/// Converts web mercator projected coordinates (XY) to geographical coordinates (LngLat)
pub fn convert_lng_lat(xy: XY) -> LngLat {
    let lng = xy.x * R2D / RE;
    let lat = ((PI * 0.5) - 2.0 * (-xy.y / RE).exp().atan()) * R2D;

    LngLat { lng, lat }
}

/// Get neighbor tiles for specific tiles
pub fn get_neighbors(tile: Tile) -> Vec<Tile> {
    let mut tiles = Vec::new();

    let (_, hi) = minmax(tile.z);

    for i in [-1, 0, 1].iter() {
        for j in [-1, 0, 1].iter() {
            let (i, j) = (*i, *j);
            if i == 0 && j == 0 {
                continue;
            } else if tile.x + i < 0 || tile.y + j < 0 {
                continue;
            } else if tile.x + i > hi || tile.y + j > hi {
                continue;
            }
            tiles.push(Tile::new(tile.x + i, tile.y + j, tile.z));
        }
    }

    fn valid(tile: &Tile) -> bool {
        let validx = 0 <= tile.x && tile.x <= 2_i32.pow(tile.z as u32) - 1;
        let validy = 0 <= tile.y && tile.y <= 2_i32.pow(tile.z as u32) - 1;
        let validz = 0 <= tile.z;
        validx && validy && validz
    }

    tiles.into_iter().filter(|t| valid(t)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ul() {
        let tile: Tile = Tile::new(486, 332, 10);
        let result = ul(tile);
        let expected_result = LngLat { lng: -9.140625, lat: 53.33087298301705 };

        assert_eq!(result, expected_result)
    }

    #[test]
    fn test_bounds() {
        let tile: Tile = Tile::new(486, 332, 10);
        let result = bounds(tile);
        let expected_result = LngLatBbox {
            west: -9.140625,
            south: 53.120405283106564,
            east: -8.7890625,
            north: 53.33087298301705,
        };

        assert_eq!(result, expected_result)
    }

    #[test]
    fn test_xy_bounds() {
        let tile: Tile = Tile::new(486, 332, 10);
        let result = xy_bounds(tile);
        let expected_result = Bbox {
            left: -1017529.7205322646,
            bottom: 7005300.768279833,
            right: -978393.9620502543,
            top: 7044436.526761843,
        };

        assert_eq!(result, expected_result)
    }

    #[test]
    fn test_xy_positive() {
        let lng_lat = LngLat { lng: -9.140625, lat: 53.33087298301705 };
        let result = convert_xy(lng_lat);
        let expected = XY { x: -1017529.7205322663, y: 7044436.526761846 };
        assert_eq!(result, expected);
    }

    #[test]
    fn test_xy_negative() {
        let lng_lat = LngLat { lng: -122.4194, lat: -100.0 }; // Latitude less than -90
        let result = convert_xy(lng_lat);
        let expected = XY { x: -13627665.271218073, y: f64::NEG_INFINITY };
        assert_eq!(result, expected);
    }

    #[test]
    fn test_xy_positive_infinity() {
        let lng_lat = LngLat { lng: -122.4194, lat: 100.0 }; // Latitude greater than 90
        let result = convert_xy(lng_lat);
        let expected = XY {
            x: -13627665.271218073,
            y: f64::INFINITY,
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn test_lng_lat() {
        let xy = XY { x: -1017529.7205322663, y: 7044436.526761846 };
        let result = convert_lng_lat(xy);
        let expected = LngLat { lng: -9.140625000000002, lat: 53.33087298301706 };
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_neighbors() {
        let tile = Tile::new(486, 332, 10);

        let result = get_neighbors(tile);

        let expected = vec![
            Tile::new(485, 331, 10),
            Tile::new(485, 332, 10),
            Tile::new(485, 333, 10),
            Tile::new(486, 331, 10),
            Tile::new(486, 333, 10),
            Tile::new(487, 331, 10),
            Tile::new(487, 332, 10),
            Tile::new(487, 333, 10),
        ];

        assert_eq!(result, expected);
    }
}
