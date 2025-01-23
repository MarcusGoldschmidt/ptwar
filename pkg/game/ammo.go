package game

type AmmoType int

const (
	AmmoTypeBullet AmmoType = iota
	AmmoTypeShell
	AmmoTypeRocket
	AmmoTypeGrenade
	AmmoTypeMissile
	AmmoTypeFire
)

type AmmoData struct {
	Id          uint64
	Name        string
	ShortName   string
	Description string

	Weight float32
	Type   AmmoType
}

type Ammo struct {
	AmmoData *AmmoData
	Count    int
}

func (a *Ammo) TotalWeight() float32 {
	return a.AmmoData.Weight * float32(a.Count)
}

type AmmoBag map[*AmmoData]*Ammo

func (a AmmoBag) TotalWeight() float32 {
	var total float32
	for _, ammo := range a {
		total += ammo.TotalWeight()
	}
	return total
}

func (a AmmoBag) GetAmmo(ammoData *AmmoData) *Ammo {
	if ammo, ok := a[ammoData]; ok {
		return ammo
	}

	return nil
}
