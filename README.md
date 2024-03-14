# Mercantile

Mercantile is a Rust library providing utility functions for working with geographical coordinates, web mercator projected coordinates, and map tiles. It includes functions for converting between latitude/longitude (LngLat) coordinates and web mercator projected coordinates (XY), as well as functions for working with map tiles and their bounding boxes.

## Features

- Convert between LngLat and XY coordinates.
- Calculate upper-left coordinates of a tile.
- Calculate bounding boxes of tiles in both geographical and XY coordinates.
- Provides utility functions for working with map tiles.

## Installation

Add Mercantile as a dependency in your `Cargo.toml` file:

```toml
[dependencies]
mercantile = "0.1.0"
```

## Usage
```rust
use mercantile::*;

fn main() {
    // Create a new tile object
    let tile = Tile::new(486, 332, 10);
    
    // Calculate the upper-left geographical coordinates of the tile
    let ul_coordinates = ul(tile);
    println!("Upper-left coordinates: {:?}", ul_coordinates);
    
    // Calculate the bounding box of the tile in geographical coordinates
    let bbox = bounds(tile);
    println!("Bounding box: {:?}", bbox);
    
    // Calculate the bounding box of the tile in web mercator coordinates
    let xy_bbox = xy_bounds(tile);
    println!("XY Bounding box: {:?}", xy_bbox);
    
    // Convert LngLat coordinates to XY coordinates
    let lng_lat = LngLat { lng: -9.140625, lat: 53.33087298301705 };
    let xy_coordinates = xy(lng_lat);
    println!("XY coordinates: {:?}", xy_coordinates);
    
    // Convert XY coordinates back to LngLat coordinates
    let xy_coordinates = XY { x: -1017529.7205322663, y: 7044436.526761846 };
    let lng_lat = lng_lat(xy_coordinates);
    println!("LngLat coordinates: {:?}", lng_lat);
}
```