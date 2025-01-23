package world

import (
	"github.com/stretchr/testify/require"
	"testing"
)

func TestGenerateMap(t *testing.T) {
	region := GenerateRegionTile(1)

	require.NotNil(t, region)
	require.NotNil(t, region.HexLayout)
	require.NotNil(t, region.Tiles)

	require.Equal(t, 7, len(region.Tiles))
}

func BenchmarkGenerateMap(b *testing.B) {
	for i := 0; i < b.N; i++ {
		region := GenerateRegionTile(1000)

		require.NotNil(b, region)
		require.NotNil(b, region.HexLayout)
		require.NotNil(b, region.Tiles)
	}
}
