package world

import (
	"github.com/stretchr/testify/require"
	"testing"
)

func TestGenerateImage(t *testing.T) {
	region := GenerateRegionTile(1)

	img, err := region.GenerateImage(4)
	require.NoError(t, err)

	require.NotNil(t, img)
}
