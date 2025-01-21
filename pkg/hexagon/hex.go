package hexagon

// Hexagons implementation interpreted from
// https://www.redblobgames.com/grids/hexagons/implementation.html
// and
// https://www.redblobgames.com/grids/hexagons/

import (
	"math"
	"ptwar/pkg/shared"
)

// Diagonal represents the direction of each point on a hex.
type Diagonal int

// String returns the string name of the direction.
func (d Diagonal) String() string {
	ret := "DiagonalUndefined"
	switch d {
	case DiagonalPosQ:
		ret = "DiagonalPosQ"
	case DiagonalPosR:
		ret = "DiagonalPosR"
	case DiagonalPosS:
		ret = "DiagonalPosS"
	case DiagonalNegQ:
		ret = "DiagonalNegQ"
	case DiagonalNegR:
		ret = "DiagonalNegR"
	case DiagonalNegS:
		ret = "DiagonalNegS"
	default:
		panic("unhandled default case")
	}
	return ret
}

// Constants for the ddiagonal from a Hex
const (
	DiagonalPosQ Diagonal = iota
	DiagonalNegR
	DiagonalPosS
	DiagonalNegQ
	DiagonalPosR
	DiagonalNegS
	DiagonalUndefined
)

// Hex is a single hexagon in the grid.
type Hex struct {
	Q, R int
}

// Delta converts the hex to a delta.
func (h Hex) Delta() Delta {
	return Delta{h.Q, h.R, -h.Q - h.R}
}

// Neighbor one step in a specific direction.
func (h Hex) Neighbor(d DirectionEnum) Hex {
	return Add(h, NeighborDelta(d))
}

// Float returns the cube coordinates as float values.
func (h Hex) Float() (float64, float64, float64) {
	return float64(h.Q), float64(-h.Q - h.R), float64(h.R)
}

// Delta is the amount of change between two hexagons.
type Delta struct {
	Q, R, S int
}

// Hex converts the delta to a hex.
func (d Delta) Hex() Hex {
	return Hex{d.Q, d.R}
}

// Abs returns the delta as absolute values. Cmath.Abs(delta)
func (d Delta) Abs() Delta {
	return Delta{
		int(math.Abs(float64(d.Q))),
		int(math.Abs(float64(d.R))),
		int(math.Abs(float64(d.S))),
	}
}

// Add is (a + b)
func Add(a Hex, b Delta) Hex {
	return Hex{
		Q: a.Q + b.Q,
		R: a.R + b.R,
	}
}

// Subtract the coordinates of the second hexagon from the first hexagon. (a - b)
func Subtract(a, b Hex) Delta {
	return Delta{
		Q: a.Q - b.Q,
		R: a.R - b.R,
		S: -(a.Q - b.Q) - (a.R - b.R),
	}
}

// Multiply a delta by a fixed amount (x(a))
func Multiply(d Delta, k int) Delta {
	return Delta{d.Q * k, d.R * k, d.S * k}
}

// RotateClockwise rotates one point around another point clockwise
func RotateClockwise(origin, moving Hex) Hex {
	before := Subtract(moving, origin)
	after := Delta{-before.R, -before.S, -before.Q}
	return Add(origin, after)
}

// RotateCounterClockwise rotates one point around another point counter clockwise
func RotateCounterClockwise(origin, moving Hex) Hex {
	before := Subtract(moving, origin)
	after := Delta{-before.S, -before.Q, -before.R}
	return Add(origin, after)
}

// Length returns the manhattan distance for a delta
func Length(d Delta) int {
	abs := d.Abs()
	return (abs.Q + abs.R + abs.S) >> 1
}

// Direction returns the Direction one point is in comparison to another point.
func Direction(d Delta) DirectionEnum {
	abs := d.Abs()
	if abs.Q >= abs.R && abs.Q >= abs.S {
		if d.Q < 0 {
			return DirectionNegQ
		}
		return DirectionPosQ
	}
	if abs.R >= abs.S {
		if d.R < 0 {
			return DirectionNegR
		}
		return DirectionPosR
	}
	if d.S < 0 {
		return DirectionNegS
	}
	return DirectionPosS
}

// DirectionEnum represents the directions of each of the sides of a hex.
type DirectionEnum int

// String returns the string name of the direction.
func (d DirectionEnum) String() string {
	ret := "DirectionUndefined"
	switch d {
	case DirectionPosQ:
		ret = "DirectionPosQ"
	case DirectionPosR:
		ret = "DirectionPosR"
	case DirectionPosS:
		ret = "DirectionPosS"
	case DirectionNegQ:
		ret = "DirectionNegQ"
	case DirectionNegR:
		ret = "DirectionNegR"
	case DirectionNegS:
		ret = "DirectionNegS"
	}
	return ret
}

// Constants for the directions from a Hex.
const (
	DirectionPosQ DirectionEnum = iota
	DirectionNegR
	DirectionPosS
	DirectionNegQ
	DirectionPosR
	DirectionNegS
	DirectionUndefined
)

var neighbors = []Delta{
	{1, 0, -1}, {1, -1, 0}, {0, -1, 1}, // positive
	{-1, 0, 1}, {-1, 1, 0}, {0, 1, -1}, // negative
	{}, // undefined
}

// NeighborDelta returns the delta required to move a single hex in a direction.
func NeighborDelta(d DirectionEnum) Delta {
	return neighbors[d]
}

var diagonals = []Delta{
	{2, -1, -1}, {1, -2, 1}, {-1, -1, 2}, // positive
	{-2, 1, 1}, {-1, 2, -1}, {1, 1, -2}, // negative
	{}, // undefined
}

// DiagonalDelta returns the delta required to move a single hex in a direction.
func DiagonalDelta(d DirectionEnum) Delta {
	return diagonals[d]
}

// Line gets the hexagons on a line between two hex.
func Line(a, b Hex) []Hex {
	delta := Subtract(a, b)
	n := Length(delta)
	dir := Direction(delta)

	results := make([]Hex, 0, n)
	visited := make(map[Hex]bool, n)
	ax, ay, az := a.Float()
	bx, by, bz := b.Float()
	x, y, z := bx-ax, by-ay, bz-az

	step := 1. / float64(n)
	for h := 0; h <= n; h++ {
		t := step * float64(h)
		pnt := unfloat(ax+x*t, ay+y*t, az+z*t)
		for visited[pnt] {
			pnt = pnt.Neighbor(dir)
		}
		results = append(results, pnt)
		visited[pnt] = true
	}
	if !visited[b] {
		results = append(results, b)
	}

	return results
}

// Range returns the slice of all points in a distance from a point.
func Range(h Hex, rad int) map[Hex]bool {
	results := make(map[Hex]bool, rad*rad)
	if rad < 1 {
		return results
	}
	for x := -rad; x <= rad; x++ {
		for y := intMax(-rad, -x-rad); y <= intMin(rad, -x+rad); y++ {
			z := -x - y
			delta := Delta{
				Q: x,
				R: z,
				S: y,
			}
			results[Add(h, delta)] = true
		}
	}
	return results
}

// Ring returns the ring of hex points specific manhattan distance from h.
func Ring(h Hex, rad int) map[Hex]bool {
	results := make(map[Hex]bool)
	if rad < 1 {
		return results
	}

	h = Add(h, Multiply(NeighborDelta(DirectionPosS), rad))
	results[h] = true
	if rad > 1 {
		for i := 0; i < 6; i++ {
			for j := 0; j < rad; j++ {
				h = Add(h, NeighborDelta(DirectionEnum(i)))
				results[h] = true
			}
		}
	}
	return results
}

// unfloat returns a tuple as a Point, Rounded.
func unfloat(x, y, z float64) Hex {
	rx, ry, rz := math.Round(x), math.Round(y), math.Round(z)
	dx, dy, dz := math.Abs(rx-x), math.Abs(ry-y), math.Abs(rz-z)

	if dx > dz && dx > dy {
		rx = -rz - ry
	} else if dz > dy {
		rz = -rx - ry
	} else {
		ry = -rx - rz
	}
	return Hex{
		Q: int(math.Round(rx)),
		R: int(math.Round(rz)),
	}
}

func intMax(a, b int) int {
	if a < b {
		return b
	}
	return a
}

func intMin(a, b int) int {
	if a < b {
		return a
	}
	return b
}

// Orientation is the orientation of the hexagon map
type Orientation struct {
	f, b [4]float64
	a    float64
	c    [6]float64
	s    [6]float64
}

// Define the default set of orientations.
var (
	OrientationPointy = Orientation{
		f: [4]float64{math.Sqrt(3.), math.Sqrt(3.) / 2., 0.0, 3. / 2.},
		b: [4]float64{math.Sqrt(3.) / 3., -1. / 3., 0.0, 2. / 3.},
		a: 0.5,
		c: [6]float64{
			math.Cos(2. * math.Pi * 0.5 / 6),
			math.Cos(2. * math.Pi * 1.5 / 6),
			math.Cos(2. * math.Pi * 2.5 / 6),
			math.Cos(2. * math.Pi * 3.5 / 6),
			math.Cos(2. * math.Pi * 4.5 / 6),
			math.Cos(2. * math.Pi * 5.5 / 6),
		},
		s: [6]float64{
			math.Sin(2. * math.Pi * 0.5 / 6),
			math.Sin(2. * math.Pi * 1.5 / 6),
			math.Sin(2. * math.Pi * 2.5 / 6),
			math.Sin(2. * math.Pi * 3.5 / 6),
			math.Sin(2. * math.Pi * 4.5 / 6),
			math.Sin(2. * math.Pi * 5.5 / 6),
		},
	}
	OrientationFlat = Orientation{
		f: [4]float64{3. / 2., 0.0, math.Sqrt(3.) / 2., math.Sqrt(3.)},
		b: [4]float64{2. / 3., 0.0, -1. / 3., math.Sqrt(3.) / 3.},
		a: 0.0,
		c: [6]float64{
			math.Cos(2. * math.Pi * 0. / 6),
			math.Cos(2. * math.Pi * 1. / 6),
			math.Cos(2. * math.Pi * 2. / 6),
			math.Cos(2. * math.Pi * 3. / 6),
			math.Cos(2. * math.Pi * 4. / 6),
			math.Cos(2. * math.Pi * 5. / 6),
		},
		s: [6]float64{
			math.Sin(2. * math.Pi * 0. / 6),
			math.Sin(2. * math.Pi * 1. / 6),
			math.Sin(2. * math.Pi * 2. / 6),
			math.Sin(2. * math.Pi * 3. / 6),
			math.Sin(2. * math.Pi * 4. / 6),
			math.Sin(2. * math.Pi * 5. / 6),
		},
	}
)

// Layout is the layout of the hex grid.
type Layout struct {
	Radius shared.Vec2D // Radius is the radius of the hexagon; supports stretching on X or Y.
	Origin shared.Vec2D // Origin is the where the center of Hex{0, 0} will be displayed.
	m      Orientation
}

// MakeLayout for rendering on the screen.
func MakeLayout(hexSize shared.Vec2D, originCenter shared.Vec2D, orientation Orientation) Layout {
	return Layout{
		Radius: hexSize,
		Origin: originCenter,
		m:      orientation,
	}
}

// CenterFor returns the point at the center (as a float) of the hex based on the layout.
func (l Layout) CenterFor(h Hex) shared.Vec2D {
	q, r :=
		float64(h.Q),
		float64(h.R)
	x := (l.m.f[0]*q + l.m.f[1]*r) * l.Radius.X
	y := (l.m.f[2]*q + l.m.f[3]*r) * l.Radius.Y
	return shared.NewVec2D(x+l.Origin.X, y+l.Origin.Y)
}

// HexFor for a hex.F that represents a point where things are laid out.
func (l Layout) HexFor(f shared.Vec2D) Hex {
	x, y :=
		f.X-l.Origin.X,
		f.Y-l.Origin.Y
	q := (l.m.b[0]*x + l.m.b[1]*y) / l.Radius.X
	r := (l.m.b[2]*x + l.m.b[3]*y) / l.Radius.Y
	return unfloat(q, -q-r, r)
}

// RingFor returns a set of hex within rad pixel distance of center.
func (l Layout) RingFor(center Hex, rad float64) map[Hex]bool {
	result := make(map[Hex]bool, 1)
	if rad < l.Radius.X && rad < l.Radius.Y {
		result[center] = true
		return result
	}
	cp := l.CenterFor(center)
	P := 1 - rad
	pxl := shared.NewVec2D(rad, 0)
	for ; pxl.X > pxl.Y; pxl.Y++ {
		if P <= 0 {
			P = P + 2*pxl.Y + 1
		} else {
			pxl.X--
			P = P + 2*pxl.Y - 2*pxl.X + 1
		}

		if pxl.X < pxl.Y {
			break
		}

		points := []shared.Vec2D{
			{pxl.X + cp.X, pxl.Y + cp.Y},
			{-pxl.X + cp.X, pxl.Y + cp.Y},
			{pxl.X + cp.X, -pxl.Y + cp.Y},
			{-pxl.X + cp.X, -pxl.Y + cp.Y},
			{pxl.Y + cp.X, pxl.X + cp.Y},
			{-pxl.Y + cp.X, pxl.X + cp.Y},
			{pxl.Y + cp.X, -pxl.X + cp.Y},
			{-pxl.Y + cp.X, -pxl.X + cp.Y},
		}
		for _, v := range points {
			result[l.HexFor(v)] = true
		}
	}
	return result
}

// AreaFor returns all hex in the area of a screen circle.
func (l Layout) AreaFor(center Hex, rad float64) map[Hex]bool {
	loop := l.RingFor(center, rad)
	result := make(map[Hex]bool)
	for k, v := range loop {
		if v == true {
			result[k] = true
			for _, inside := range Line(k, center) {
				result[inside] = true
			}
		}
	}
	return result
}

// Vertices returns the location of all verticies for a given hexagon.
func (l Layout) Vertices(h Hex) []shared.Vec2D {
	result := make([]shared.Vec2D, 6, 7)
	center := l.CenterFor(h)
	for k := range result {
		result[k] = shared.Vec2D{
			X: center.X + l.Radius.X*l.m.c[k],
			Y: center.Y + l.Radius.Y*l.m.s[k],
		}
	}
	result = append(result, center)
	return result
}
