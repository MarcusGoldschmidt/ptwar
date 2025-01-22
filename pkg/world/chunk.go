package world

import (
	"github.com/MarcusGoldschmidt/ptwar/pkg/hexagon"
	"github.com/MarcusGoldschmidt/ptwar/pkg/shared"
)

type Tile struct {
}

func GenerateMap(hexagonRadius float64) *Tile {
	radius := shared.Vec2D{X: hexagonRadius, Y: hexagonRadius}

	hexagon.MakeLayout(radius, shared.Vec2D{}, hexagon.OrientationFlat)

	return &Tile{}
}
