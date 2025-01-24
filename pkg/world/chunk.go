package world

import (
	"github.com/MarcusGoldschmidt/ptwar/pkg/hexagon"
	"github.com/MarcusGoldschmidt/ptwar/pkg/procedural"
	"github.com/MarcusGoldschmidt/ptwar/pkg/shared"
	"math/rand"
)

//go:generate stringer -type=TerrainType
type TerrainType int

const (
	Grass TerrainType = iota
	Water
	DeepWater
	Desert
	Forest
	DeepForest
	Mountain
	HighMountain
	Snow
	Ice
)

func NewTerrainType(v float64) TerrainType {
	// TODO: Make this more sophisticated
	switch {
	case v < 0.1:
		return Water
	case v < 0.2:
		return DeepWater
	case v < 0.3:
		return Desert
	case v < 0.4:
		return Forest
	case v < 0.5:
		return DeepForest
	case v < 0.6:
		return Mountain
	case v < 0.7:
		return HighMountain
	case v < 0.8:
		return Snow
	case v < 0.9:
		return Ice
	default:
		return Grass
	}
}

type Tile struct {
	Position hexagon.Hex
	Terrain  TerrainType
	Noise    float64
}

type RegionTile struct {
	HexLayout hexagon.Layout
	Tiles     map[hexagon.Hex]*Tile
}

func GenerateRegionTile(hexagonRadius float64) *RegionTile {
	noiseMap := procedural.PerlinNoise(procedural.NoiseOptions{
		Seed:  rand.Int63(),
		Scale: 1,
	})

	radius := shared.Vec2D{X: hexagonRadius, Y: hexagonRadius}

	l := hexagon.MakeLayout(radius, shared.Vec2D{}, hexagon.OrientationFlat)

	hexes := l.AllHex()

	tiles := make(map[hexagon.Hex]*Tile, len(hexes))

	for _, hex := range hexes {
		noise := noiseMap.Get(hex.Q, hex.R)

		tiles[hex] = &Tile{
			Position: hex,
			Noise:    noise,
			Terrain:  NewTerrainType(noise),
		}
	}

	return &RegionTile{
		HexLayout: l,
		Tiles:     tiles,
	}
}
