package procedural

func PerlinNoise(option NoiseOptions) Noise {
	noiseMaker := PerlinNoiseMaker(option.Seed, option.Scale)

	return Noise{
		Seed:       option.Seed,
		noiseMaker: noiseMaker,
	}
}
