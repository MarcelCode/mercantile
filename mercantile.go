package mercantile

import (
	"math"
)

const R2D = 180 / math.Pi
const RE = 6378137.0
const CE = 2 * math.Pi * RE

type Tile struct {
	x float64
	y float64
	z float64
}

type Bbox struct {
	west  float64
	south float64
	east  float64
	north float64
}

func radToDegrees(radians float64) float64 {
	return radians * (180 / math.Pi)
}

func degToRadians(degrees float64) float64 {
	return degrees * (math.Pi / 180)
}

func ul(tile Tile) (lng, lat float64) {
	Z2 := math.Pow(2, tile.z)

	lng = tile.x/Z2*360.0 - 180.0
	lat = radToDegrees(math.Atan(math.Sinh(math.Pi * (1 - 2*tile.y/Z2))))

	return
}

func bounds(tile Tile) Bbox {
	Z2 := math.Pow(2, tile.z)

	ulLonDeg := tile.x/Z2*360.0 - 180.0
	ulLatRad := math.Atan(math.Sinh(math.Pi * (1 - 2*tile.y/Z2)))
	ulLatDeg := radToDegrees(ulLatRad)

	lrLonDeg := (tile.x+1)/Z2*360.0 - 180.0
	lrLatRad := math.Atan(math.Sinh(math.Pi * (1 - 2*(tile.y+1)/Z2)))
	lrLatDeg := radToDegrees(lrLatRad)

	return Bbox{ulLonDeg, lrLatDeg, lrLonDeg, ulLatDeg}
}

func xy(lng, lat float64) (x, y float64) {
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

func lngLat(x, y float64) (lng, lat float64) {
	lng = x * R2D / RE
	lat = ((math.Pi * 0.5) - 2.0*math.Atan(math.Exp(-y/RE))) * R2D

	return
}

func xyBounds(tile Tile) Bbox {
	tileSize := CE / math.Pow(2, tile.z)
	west := tile.x*tileSize - CE/2
	east := west + tileSize
	north := CE/2 - tile.y*tileSize
	south := north - tileSize

	return Bbox{west, south, east, north}
}
