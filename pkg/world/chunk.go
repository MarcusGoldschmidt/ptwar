package world

import (
	"github.com/MarcusGoldschmidt/ptwar/pkg/hexagon"
	"github.com/MarcusGoldschmidt/ptwar/pkg/shared"
)

type Tile struct {
	Position hexagon.Hex
}

type RegionTile struct {
	HexLayout hexagon.Layout
	Tiles     map[hexagon.Hex]*Tile
}

func GenerateRegionTile(hexagonRadius float64) *RegionTile {
	radius := shared.Vec2D{X: hexagonRadius, Y: hexagonRadius}

	l := hexagon.MakeLayout(radius, shared.Vec2D{}, hexagon.OrientationFlat)

	hexes := l.AllHex()

	tiles := make(map[hexagon.Hex]*Tile)
	for _, hex := range hexes {
		tiles[hex] = &Tile{Position: hex}
	}

	return &RegionTile{
		HexLayout: l,
		Tiles:     tiles,
	}
}
