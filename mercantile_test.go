package mercantile

import (
	"reflect"
	"testing"
)

func AssertEqual(t *testing.T, a interface{}, b interface{}) {
	if a == b {
		return
	}
	t.Errorf("Received %v (type %v), expected %v (type %v)", a, reflect.TypeOf(a), b, reflect.TypeOf(b))
}

func TestUl(t *testing.T) {
	testTile := Tile{486, 332, 10}

	resultX, resultY := Ul(testTile)
	expectedX, expectedY := -9.140625, 53.330872983017045

	AssertEqual(t, resultX, expectedX)
	AssertEqual(t, resultY, expectedY)
}

func TestBounds(t *testing.T) {
	testTile := Tile{486, 332, 10}

	result := Bounds(testTile)
	expected := Bbox{-9.140625, 53.12040528310657, -8.7890625, 53.330872983017045}

	AssertEqual(t, result, expected)
}

func TestXy(t *testing.T) {
	lng, lat := -9.140625, 53.33087298301705

	resultX, resultY := Xy(lng, lat)
	expectedX, expectedY := -1017529.7205322663, 7044436.526761846

	AssertEqual(t, resultX, expectedX)
	AssertEqual(t, resultY, expectedY)
}

func TestLngLat(t *testing.T) {
	x, y := -1017529.7205322663, 7044436.526761846

	resultLng, resultLat := LngLat(x, y)
	expectedLng, expectedLat := -9.140625000000002, 53.330872983017066

	AssertEqual(t, resultLng, expectedLng)
	AssertEqual(t, resultLat, expectedLat)
}

func TestXyBounds(t *testing.T) {
	testTile := Tile{486, 332, 10}

	resultBbox := XyBounds(testTile)
	expectedBbox := Bbox{-1.0175297205322646e+06, 7.005300768279833e+06,
		-978393.9620502543, 7.044436526761843e+06}

	AssertEqual(t, resultBbox, expectedBbox)
}
