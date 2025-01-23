package procedural

import (
	"github.com/ojrac/opensimplex-go"
	"time"
)

func PerlinNoise(option NoiseOptions) Noise {
	seed := time.Now().UnixNano()
	if option.Seed != 0 {
		seed = option.Seed
	}

	noise := opensimplex.New(seed)

	// Create a 2D slice to hold the tiles
	mapData := make([][]float64, option.Width)

	for x := 0; x < option.Width; x++ {
		mapData[x] = make([]float64, option.Height)
		for y := 0; y < option.Height; y++ {
			mapData[x][y] = noise.Eval2(float64(x)*option.Scale, float64(y)*option.Scale)
		}
	}

	return Noise{
		Seed:   seed,
		Height: option.Height,
		Width:  option.Width,
		Value:  mapData,
	}
}
