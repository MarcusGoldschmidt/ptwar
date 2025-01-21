package world

import "ptwar/pkg/hexagon"

type Tile struct {
}

func GenerateMap(hexagonRadius float64) *Tile {
	radius := hexagon.F{X: hexagonRadius, Y: hexagonRadius}

	hexagon.MakeLayout(radius, hexagon.F{}, hexagon.OrientationFlat)

	return &Tile{}
}
