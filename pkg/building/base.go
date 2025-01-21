package building

type BuildState int

const (
	BuildStateNone BuildState = iota
	BuildStateBuilding
	BuildStateComplete
	BuildStateDestroyed
)

type Building interface {
	Id() uint64
	Description() string
	BuildState() BuildState
	Life() float64
}
