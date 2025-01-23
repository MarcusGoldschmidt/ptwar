package shared

type Comparable interface {
	IsEqualTo(other interface{}) bool
	IsGreaterThan(other interface{}) bool
}
