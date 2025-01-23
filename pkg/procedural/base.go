package procedural

type NoiseOptions struct {
	Seed   int64
	Height int
	Width  int
	Scale  float64
}

type Noise struct {
	Seed   int64
	Height int
	Width  int

	Value [][]float64
}
