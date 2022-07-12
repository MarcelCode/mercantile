package mercantile

import (
	"fmt"
	"math"
)

const R2D = 180 / math.Pi
const RE = 6378137.0
const CE = 2 * math.Pi * RE

// Tile An XYZ web mercator tile
type Tile struct {
	X int
	Y int
	Z int
}

// Bbox A web mercator bounding box
type Bbox struct {
	MinX float64
	MinY float64
	MaxX float64
	MaxY float64
}

func (bbox *Bbox) AsString() (StrMinX string, StrMinY string, StrMaxX string, StrMaxY string) {
	StrMinX = fmt.Sprintf("%f", bbox.MinX)
	StrMinY = fmt.Sprintf("%f", bbox.MinY)
	StrMaxX = fmt.Sprintf("%f", bbox.MaxX)
	StrMaxY = fmt.Sprintf("%f", bbox.MaxY)

	return
}

func radToDegrees(radians float64) float64 {
	return radians * (180 / math.Pi)
}

func degToRadians(degrees float64) float64 {
	return degrees * (math.Pi / 180)
}

// Ul Returns the upper left longitude and latitude of a tile
func Ul(tile Tile) (lng, lat float64) {
	Z2 := math.Pow(2, float64(tile.Z))

	lng = float64(tile.X)/Z2*360.0 - 180.0
	lat = radToDegrees(math.Atan(math.Sinh(math.Pi * (1 - 2*float64(tile.Y)/Z2))))

	return
}

// Bounds Returns the bounding box of a tile
func Bounds(tile Tile) Bbox {
	Z2 := math.Pow(2, float64(tile.Z))

	minLngDeg := float64(tile.X)/Z2*360.0 - 180.0
	maxLatRad := math.Atan(math.Sinh(math.Pi * (1 - 2*float64(tile.Y)/Z2)))
	maxLatDeg := radToDegrees(maxLatRad)

	maxLngDeg := (float64(tile.X)+1)/Z2*360.0 - 180.0
	minLatRad := math.Atan(math.Sinh(math.Pi * (1 - 2*(float64(tile.Y)+1)/Z2)))
	minLatDeg := radToDegrees(minLatRad)

	return Bbox{minLngDeg, minLatDeg, maxLngDeg, maxLatDeg}
}

// Xy Convert longitude, latitude to web mercator x, y
func Xy(lng, lat float64) (x, y float64) {
	x = RE * degToRadians(lng)

	if lat <= -90 {
		y = math.Inf(0)
	} else if lat >= 90 {
		y = math.Inf(1)
	} else {
		y = RE * math.Log(math.Tan((math.Pi*0.25)+(0.5*degToRadians(lat))))
	}

	return
}

// LngLat Convert web mercator x, y to longitude and latitude
func LngLat(x, y float64) (lng, lat float64) {
	lng = x * R2D / RE
	lat = ((math.Pi * 0.5) - 2.0*math.Atan(math.Exp(-y/RE))) * R2D

	return
}

// XyBounds Get the web mercator bounding box of a tile
func XyBounds(tile Tile) Bbox {
	tileSize := CE / math.Pow(2, float64(tile.Z))
	minX := float64(tile.X)*tileSize - CE/2
	maxX := minX + tileSize
	maxY := CE/2 - float64(tile.Y)*tileSize
	minY := maxY - tileSize

	return Bbox{minX, minY, maxX, maxY}
}
