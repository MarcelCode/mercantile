package mercantile

import (
	"math"
)

const R2D = 180 / math.Pi
const RE = 6378137.0
const CE = 2 * math.Pi * RE

type Tile struct {
	X int
	Y int
	Z int
}

type Bbox struct {
	West  float64
	South float64
	East  float64
	North float64
}

func RadToDegrees(radians float64) float64 {
	return radians * (180 / math.Pi)
}

func DegToRadians(degrees float64) float64 {
	return degrees * (math.Pi / 180)
}

func Ul(tile Tile) (lng, lat float64) {
	Z2 := math.Pow(2, float64(tile.Z))

	lng = float64(tile.X)/Z2*360.0 - 180.0
	lat = RadToDegrees(math.Atan(math.Sinh(math.Pi * (1 - 2*float64(tile.Y)/Z2))))

	return
}

func Bounds(tile Tile) Bbox {
	Z2 := math.Pow(2, float64(tile.Z))

	ulLonDeg := float64(tile.X)/Z2*360.0 - 180.0
	ulLatRad := math.Atan(math.Sinh(math.Pi * (1 - 2*float64(tile.Y)/Z2)))
	ulLatDeg := RadToDegrees(ulLatRad)

	lrLonDeg := (float64(tile.X)+1)/Z2*360.0 - 180.0
	lrLatRad := math.Atan(math.Sinh(math.Pi * (1 - 2*(float64(tile.Y)+1)/Z2)))
	lrLatDeg := RadToDegrees(lrLatRad)

	return Bbox{ulLonDeg, lrLatDeg, lrLonDeg, ulLatDeg}
}

func Xy(lng, lat float64) (x, y float64) {
	x = RE * DegToRadians(lng)

	if lat <= -90 {
		y = math.Inf(0)
	} else if lat >= 90 {
		y = math.Inf(1)
	} else {
		y = RE * math.Log(math.Tan((math.Pi*0.25)+(0.5*DegToRadians(lat))))
	}

	return
}

func LngLat(x, y float64) (lng, lat float64) {
	lng = x * R2D / RE
	lat = ((math.Pi * 0.5) - 2.0*math.Atan(math.Exp(-y/RE))) * R2D

	return
}

func XyBounds(tile Tile) Bbox {
	tileSize := CE / math.Pow(2, float64(tile.Z))
	west := float64(tile.X)*tileSize - CE/2
	east := west + tileSize
	north := CE/2 - float64(tile.Y)*tileSize
	south := north - tileSize

	return Bbox{west, south, east, north}
}
