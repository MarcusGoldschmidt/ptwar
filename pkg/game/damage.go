package game

type DamageType int

const (
	DamageTypeNone DamageType = iota
	DamageTypeExplosion
	DamageTypeFire
	DamageTypePoison
	DamageTypeRadiation
	DamageTypePiercing
)

type Damage struct {
	BaseDamage float64
	Type       DamageType
}
