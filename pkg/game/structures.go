package game

type StructureType int

const (
	StructureTypeFactory StructureType = iota
	StructureTypeWarehouse
	StructureTypePowerPlant
	StructureTypeResidence
	StructureTypeDefense
	StructureTypeGun
)

type BuildState int

const (
	BuildStateBuilding BuildState = iota
	BuildStateUpgrading
	BuildStateComplete
	BuildStateDestroyed
)

type Building interface {
	Id() uint64
	Description() string
	BuildState() BuildState
	Life() float64
	Level() int
	Resistances() map[DamageType]float64
	TakeDamage(damage Damage)
}
