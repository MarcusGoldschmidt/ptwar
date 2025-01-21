package shared

import "math"

type Vec2D struct {
	X float64
	Y float64
}

func NewVec2D(x, y float64) Vec2D {
	return Vec2D{X: x, Y: y}
}

func (p Vec2D) Add(other Vec2D) Vec2D {
	return Vec2D{X: p.X + other.X, Y: p.Y + other.Y}
}

func (p Vec2D) Sub(other Vec2D) Vec2D {
	return Vec2D{X: p.X - other.X, Y: p.Y - other.Y}
}

// Math Utils

func (p Vec2D) Length() float64 {
	return p.X*p.X + p.Y*p.Y
}

func (p Vec2D) Distance(other Vec2D) float64 {
	return p.Sub(other).Length()
}

func (p Vec2D) Normalize() Vec2D {
	length := p.Length()
	if length == 0 {
		return Vec2D{}
	}
	return Vec2D{X: p.X / length, Y: p.Y / length}
}

func (p Vec2D) Scale(scalar float64) Vec2D {
	return Vec2D{X: p.X * scalar, Y: p.Y * scalar}
}

func (p Vec2D) Dot(other Vec2D) float64 {
	return p.X*other.X + p.Y*other.Y
}

func (p Vec2D) Angle(other Vec2D) float64 {
	return p.Dot(other) / (p.Length() * other.Length())
}

func (p Vec2D) Rotate(angle float64) Vec2D {
	cos := math.Cos(angle)
	sin := math.Sin(angle)
	return Vec2D{
		X: p.X*cos - p.Y*sin,
		Y: p.X*sin + p.Y*cos,
	}
}

func (p Vec2D) Lerp(other Vec2D, t float64) Vec2D {
	return Vec2D{
		X: p.X + (other.X-p.X)*t,
		Y: p.Y + (other.Y-p.Y)*t,
	}
}

func (p Vec2D) LerpClamped(other Vec2D, t float64) Vec2D {
	if t < 0 {
		return p
	}
	if t > 1 {
		return other
	}
	return p.Lerp(other, t)
}

func (p Vec2D) Reflect(normal Vec2D) Vec2D {
	return p.Sub(normal.Scale(2 * p.Dot(normal)))
}

func (p Vec2D) ProjectOnto(other Vec2D) Vec2D {
	return other.Scale(p.Dot(other) / other.Length())
}

func (p Vec2D) ProjectOntoNormalized(other Vec2D) Vec2D {
	return other.Scale(p.Dot(other))
}

func (p Vec2D) Clamp(min, max Vec2D) Vec2D {
	return Vec2D{
		X: math.Max(min.X, math.Min(max.X, p.X)),
		Y: math.Max(min.Y, math.Min(max.Y, p.Y)),
	}
}

func (p Vec2D) ClampLength(max float64) Vec2D {
	if p.Length() > max*max {
		return p.Normalize().Scale(max)
	}
	return p
}

func (p Vec2D) ClampAngle(min, max float64) Vec2D {
	angle := math.Atan2(p.Y, p.X)
	if angle < min {
		angle = min
	}
	if angle > max {
		angle = max
	}
	return Vec2D{X: math.Cos(angle), Y: math.Sin(angle)}
}

func (p Vec2D) IsZero() bool {
	return p.X == 0 && p.Y == 0
}

func (p Vec2D) IsEqual(other Vec2D) bool {
	return p.X == other.X && p.Y == other.Y
}

func (p Vec2D) IsEqualEpsilon(other Vec2D, epsilon float64) bool {
	return math.Abs(p.X-other.X) < epsilon && math.Abs(p.Y-other.Y) < epsilon
}

func (p Vec2D) IsParallel(other Vec2D) bool {
	return p.X*other.Y == p.Y*other.X
}

func (p Vec2D) IsPerpendicular(other Vec2D) bool {
	return p.Dot(other) == 0
}

func (p Vec2D) IsNormalized() bool {
	return math.Abs(p.Length()-1) < 0.00001
}

func (p Vec2D) IsNormalizedEpsilon(epsilon float64) bool {
	return math.Abs(p.Length()-1) < epsilon
}

func (p Vec2D) IsCollinear(other Vec2D) bool {
	return p.IsParallel(other) && p.Dot(other) > 0
}

func (p Vec2D) IsCollinearOpposite(other Vec2D) bool {
	return p.IsParallel(other) && p.Dot(other) < 0
}

func (p Vec2D) IsOrthogonal(other Vec2D) bool {
	return p.IsPerpendicular(other) && p.Dot(other) == 0
}

func (p Vec2D) IsClockwise(other Vec2D) bool {
	return p.X*other.Y-p.Y*other.X < 0
}

func (p Vec2D) IsCounterClockwise(other Vec2D) bool {
	return p.X*other.Y-p.Y*other.X > 0
}

func (p Vec2D) IsInTriangle(a, b, c Vec2D) bool {
	return p.IsClockwise(a) == p.IsClockwise(b) && p.IsClockwise(b) == p.IsClockwise(c)
}

func (p Vec2D) IsInPolygon(vertices []Vec2D) bool {
	n := len(vertices)
	for i := 0; i < n; i++ {
		if !p.IsClockwise(vertices[i]) {
			return false
		}
	}
	return true
}

func (p Vec2D) IsInCircle(center Vec2D, radius float64) bool {
	return p.Distance(center) <= radius
}

func (p Vec2D) IsInRectangle(min, max Vec2D) bool {
	return p.X >= min.X && p.X <= max.X && p.Y >= min.Y && p.Y <= max.Y
}

func (p Vec2D) IsInEllipse(center, radius Vec2D) bool {
	return (p.X-center.X)*(p.X-center.X)/(radius.X*radius.X)+(p.Y-center.Y)*(p.Y-center.Y)/(radius.Y*radius.Y) <= 1
}

func (p Vec2D) IsInSector(center Vec2D, radius, angle float64) bool {
	return p.Distance(center) <= radius && p.Angle(center) <= angle
}

func (p Vec2D) IsInRing(center Vec2D, radius, thickness float64) bool {
	return p.Distance(center) >= radius-thickness && p.Distance(center) <= radius
}
