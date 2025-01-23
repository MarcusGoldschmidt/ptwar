package procedural

import (
	"github.com/ojrac/opensimplex-go"
)

type NoiseOptions struct {
	Seed  int64
	Scale float64
}

type NoiseMaker interface {
	Make(seed int64, x, y int) float64
}

type NoiseMakerFunc func(seed int64, x, y int) float64

func (n NoiseMakerFunc) Make(seed int64, x, y int) float64 {
	return n(seed, x, y)
}

func PerlinNoiseMaker(seed int64, scale float64) NoiseMakerFunc {
	noise := opensimplex.New(seed)

	return func(seed int64, x, y int) float64 {
		return noise.Eval2(float64(x)*scale, float64(y)*scale)
	}
}

type NoiseChuck [][]float64

type Noise struct {
	Seed       int64
	noiseMaker NoiseMaker
}

func NewNoise(seed int64, noiseMaker NoiseMaker) *Noise {
	return &Noise{
		Seed:       seed,
		noiseMaker: noiseMaker,
	}
}

func (n *Noise) Get(x, y int) float64 {
	return n.noiseMaker.Make(n.Seed, x, y)
}
