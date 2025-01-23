package hexagon

import (
	"fmt"
	"github.com/MarcusGoldschmidt/ptwar/pkg/shared"
	"github.com/stretchr/testify/require"
	"testing"
)

func TestHexSize(t *testing.T) {
	for i := 0; i < 20; i++ {
		t.Run(fmt.Sprintf("TestHexSize Spiral %d", i), func(t *testing.T) {
			radius := shared.Vec2D{X: float64(i), Y: float64(i)}

			l := MakeLayout(radius, shared.Vec2D{}, OrientationFlat)

			require.Equal(t, len(l.AllHex()), l.Size())
		})
	}

	for i := 0; i < 20; i++ {
		t.Run(fmt.Sprintf("TestHexSize not Spiral %d", i), func(t *testing.T) {
			radius := shared.Vec2D{X: float64(i + 1), Y: float64(i)}

			l := MakeLayout(radius, shared.Vec2D{}, OrientationFlat)

			require.Equal(t, len(l.AllHex()), l.Size())
		})
	}
}
